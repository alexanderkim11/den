// This file is derived from the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.


use leo_ast::Stub;
use leo_compiler::Compiler;
use leo_errors::{emitter::Handler, CliError, UtilError, Result};
use leo_package::{build::BuildDirectory, outputs::OutputsDirectory, source::SourceDirectory};
use leo_retriever::{Manifest, NetworkName, Retriever};
use leo_span::{Symbol, symbol::create_session_if_not_set_then};
use leo_lang::cli::{BuildOptions, context::Context};

use snarkvm::{
    package::Package,
    prelude::{MainnetV0, Network, ProgramID, TestnetV0},
};

use indexmap::IndexMap;
use snarkvm::prelude::CanaryV0;
use std::{
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
    process::exit,
    env::current_dir,
};

fn apply(context: Context) -> Result<()>{
    // Parse the network.
    //let network = NetworkName::try_from(context.get_network(options.network)?)?;
    let network = NetworkName::try_from("testnet").unwrap();
    match network {
        NetworkName::MainnetV0 => handle_build::<MainnetV0>(context),
        NetworkName::TestnetV0 => handle_build::<TestnetV0>(context),
        NetworkName::CanaryV0 => handle_build::<CanaryV0>(context),
    }
}

// A helper function to handle the build command.
fn handle_build<N: Network>(context: Context) -> Result<()>{
    // Get the package path.
    let package_path = context.dir()?;
    let home_path = context.home()?;

    // Get the program id.
    let manifest = Manifest::read_from_dir(&package_path)?;
    let program_id = ProgramID::<N>::from_str(manifest.program())?;

    // Clear and recreate the build directory.
    let build_directory = package_path.join("build");
    if build_directory.exists() {
        std::fs::remove_dir_all(&build_directory).map_err(CliError::build_error)?;
    }
    Package::create(&build_directory, &program_id).map_err(CliError::build_error)?;

    // Initialize error handler
    let handler = Handler::default();

    // Retrieve all local dependencies in post order
    let main_sym = Symbol::intern(&program_id.name().to_string());
    let mut retriever = Retriever::<N>::new(
        main_sym,
        &package_path,
        &home_path,
        "https://api.explorer.provable.com/v1".to_string(), // context.get_endpoint(&command.options.endpoint)?.to_string(),
    )
    .map_err(|err| UtilError::failed_to_retrieve_dependencies(err, Default::default()))?;
    let mut local_dependencies =
        retriever.retrieve().map_err(|err| UtilError::failed_to_retrieve_dependencies(err, Default::default()))?;

    // Push the main program at the end of the list to be compiled after all of its dependencies have been processed
    local_dependencies.push(main_sym);

    // Recursive build will recursively compile all local dependencies. Can disable to save compile time.
    let recursive_build = true; // !command.options.non_recursive;

    // Loop through all local dependencies and compile them in order
    for dependency in local_dependencies.into_iter() {
        if recursive_build || dependency == main_sym {
            // Get path to the local project
            let (local_path, stubs) = retriever.prepare_local(dependency)?;

            // Create the outputs directory.
            let local_outputs_directory = OutputsDirectory::create(&local_path)?;

            // Open the build directory.
            let local_build_directory = BuildDirectory::create(&local_path)?;

            // Fetch paths to all .leo files in the source directory.
            let local_source_files = SourceDirectory::files(&local_path)?;

            // Check the source files.
            SourceDirectory::check_files(&local_source_files)?;

            // Compile all .leo files into .aleo files.
            for file_path in local_source_files {
                compile_leo_file(
                    file_path,
                    &ProgramID::<N>::try_from(format!("{}.aleo", dependency))
                        .map_err(|_| UtilError::snarkvm_error_building_program_id(Default::default()))?,
                    &local_outputs_directory,
                    &local_build_directory,
                    &handler,
                    Default::default(), // command.options.clone(),
                    stubs.clone(),
                )?;
            }
        }

        // Writes `leo.lock` as well as caches objects (when target is an intermediate dependency)
        retriever.process_local(dependency, recursive_build)?;
    }

    // `Package::open` checks that the build directory and that `main.aleo` and all imported files are well-formed.
    Package::<N>::open(&build_directory).map_err(CliError::failed_to_execute_build)?;

    Ok(())
}

/// Compiles a Leo file in the `src/` directory.
#[allow(clippy::too_many_arguments)]
fn compile_leo_file<N: Network>(
    file_path: PathBuf,
    program_id: &ProgramID<N>,
    outputs: &Path,
    build: &Path,
    handler: &Handler,
    options: BuildOptions,
    stubs: IndexMap<Symbol, Stub>,
) -> Result<()> {
    // Construct program name from the program_id found in `package.json`.
    let program_name = program_id.name().to_string();

    // Create the path to the Aleo file.
    let mut aleo_file_path = build.to_path_buf();
    aleo_file_path.push(format!("main.{}", program_id.network()));

    // Create a new instance of the Leo compiler.
    let mut compiler = Compiler::<N>::new(
        program_name.clone(),
        program_id.network().to_string(),
        handler,
        file_path.clone(),
        outputs.to_path_buf(),
        Some(options.into()),
        stubs,
    );

    // Compile the Leo program into Aleo instructions.
    let instructions = compiler.compile()?;

    // Write the instructions.
    std::fs::File::create(&aleo_file_path)
        .map_err(CliError::failed_to_load_instructions)?
        .write_all(instructions.as_bytes())
        .map_err(CliError::failed_to_load_instructions)?;

    // tracing::info!("✅ Compiled '{program_name}.aleo' into Aleo instructions");
    //println!("✅ Compiled '{program_name}.aleo' into Aleo instructions");
    Ok(())
}

pub fn handle_error<T>(res: Result<T>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{err}");
            exit(err.exit_code());
        }
    }
}


#[tauri::command]
pub fn compile(filepath: String){
    let filepath_buf = PathBuf::from(&filepath);
    let mut aleo_path = current_dir().unwrap();
    aleo_path.push(".aleo");
    let context = handle_error(Context::new(Some(filepath_buf), Some(aleo_path), false));
    create_session_if_not_set_then(|_| {
        let _ = apply(context).unwrap();
    });
}

