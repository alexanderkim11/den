use leo_errors::LeoError;
pub use snarkvm::console::{
    account::{Address, PrivateKey, Signature, ViewKey},
    program::{Ciphertext, Record},
};
use aleo_std::StorageMode;
use snarkvm::{
    circuit::{AleoV0, Aleo, AleoTestnetV0},
    prelude::{
        Process,
        ProgramID,
        VM,
        store::{
            ConsensusStore,
            helpers::memory::ConsensusMemory,
        },
        Network,
        Program as SnarkVMProgram,
    }
};
use leo_retriever::{fetch_from_network, verify_valid_program};
use leo_package::package::Package;



use rand::{rngs::StdRng, SeedableRng};
use std::str::FromStr;

#[tauri::command]
pub fn new_account(_handle: tauri::AppHandle, network: String) -> (bool, (String, String, String)) {
    if network == "Mainnet".to_string() {
        pub use snarkvm::console::network::MainnetV0 as CurrentNetwork;

        pub type PrivateKeyNative = PrivateKey<CurrentNetwork>;
        pub type ViewKeyNative = ViewKey<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let native_privatekey;
        let native_privatekey_result = PrivateKeyNative::new(&mut StdRng::from_entropy());
        match native_privatekey_result {
            Ok(v) => {
                native_privatekey = v;
            }
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not generate private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::try_from(&native_privatekey);
        match native_viewkey_result {
            Ok(v) => {
                native_viewkey = v;
            }
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not derive view key from private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        let native_address;
        let native_address_result = AddressNative::try_from(&native_privatekey);
        match native_address_result {
            Ok(v) => native_address = v,
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not derive address from private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        return (
            false,
            (
                native_privatekey.to_string(),
                native_viewkey.to_string(),
                native_address.to_string(),
            ),
        );
    } else if network == "Testnet".to_string() {
        pub use snarkvm::console::network::TestnetV0 as CurrentNetwork;
        pub type PrivateKeyNative = PrivateKey<CurrentNetwork>;
        pub type ViewKeyNative = ViewKey<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let native_privatekey;
        let native_privatekey_result = PrivateKeyNative::new(&mut StdRng::from_entropy());
        match native_privatekey_result {
            Ok(v) => {
                native_privatekey = v;
            }
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not generate private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::try_from(&native_privatekey);
        match native_viewkey_result {
            Ok(v) => {
                native_viewkey = v;
            }
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not derive view key from private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        let native_address;
        let native_address_result = AddressNative::try_from(&native_privatekey);
        match native_address_result {
            Ok(v) => native_address = v,
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not derive address from private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        return (
            false,
            (
                native_privatekey.to_string(),
                native_viewkey.to_string(),
                native_address.to_string(),
            ),
        );
    } else {
        return (
            true,
            (
                "Error: undefined behavior".to_string(),
                String::new(),
                String::new(),
            ),
        );
    }
}

#[tauri::command]
pub fn account_from_pk(
    _handle: tauri::AppHandle,
    network: String,
    privatekey: String,
) -> (bool, (String, String, String)) {
    if network == "Mainnet".to_string() {
        pub use snarkvm::console::network::MainnetV0 as CurrentNetwork;

        pub type PrivateKeyNative = PrivateKey<CurrentNetwork>;
        pub type ViewKeyNative = ViewKey<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let native_privatekey;
        let native_privatekey_result = PrivateKeyNative::from_str(&privatekey);
        match native_privatekey_result {
            Ok(v) => {
                native_privatekey = v;
            }
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not parse private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::try_from(&native_privatekey);
        match native_viewkey_result {
            Ok(v) => {
                native_viewkey = v;
            }
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not derive view key from private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        let native_address;
        let native_address_result = AddressNative::try_from(&native_privatekey);
        match native_address_result {
            Ok(v) => native_address = v,
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not derive address from private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        return (
            false,
            (
                native_privatekey.to_string(),
                native_viewkey.to_string(),
                native_address.to_string(),
            ),
        );
    } else if network == "Testnet".to_string() {
        pub use snarkvm::console::network::TestnetV0 as CurrentNetwork;

        pub type PrivateKeyNative = PrivateKey<CurrentNetwork>;
        pub type ViewKeyNative = ViewKey<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let native_privatekey;
        let native_privatekey_result = PrivateKeyNative::from_str(&privatekey);
        match native_privatekey_result {
            Ok(v) => {
                native_privatekey = v;
            }
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not parse private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::try_from(&native_privatekey);
        match native_viewkey_result {
            Ok(v) => {
                native_viewkey = v;
            }
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not derive view key from private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        let native_address;
        let native_address_result = AddressNative::try_from(&native_privatekey);
        match native_address_result {
            Ok(v) => native_address = v,
            Err(_) => {
                return (
                    true,
                    (
                        "Error: Could not derive address from private key".to_string(),
                        String::new(),
                        String::new(),
                    ),
                );
            }
        }

        return (
            false,
            (
                native_privatekey.to_string(),
                native_viewkey.to_string(),
                native_address.to_string(),
            ),
        );
    } else {
        return (
            true,
            (
                "Error: undefined behavior".to_string(),
                String::new(),
                String::new(),
            ),
        );
    }
}

#[tauri::command]
pub fn address_from_vk(
    _handle: tauri::AppHandle,
    network: String,
    viewkey: String,
) -> (bool, String) {
    if network == "Mainnet".to_string() {
        pub use snarkvm::console::network::MainnetV0 as CurrentNetwork;

        pub type ViewKeyNative = ViewKey<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::from_str(&viewkey);
        match native_viewkey_result {
            Ok(v) => {
                native_viewkey = v;
            }
            Err(_) => {
                return (true, "Error: Could not parse view key".to_string());
            }
        }

        let native_address;
        let native_address_result = AddressNative::try_from(&native_viewkey);
        match native_address_result {
            Ok(v) => {
                native_address = v.to_string();
            }
            Err(_) => {
                return (
                    true,
                    "Error: Could not derive address from view key".to_string(),
                );
            }
        }

        return (false, native_address);
    } else if network == "Testnet".to_string() {
        pub use snarkvm::console::network::TestnetV0 as CurrentNetwork;

        pub type ViewKeyNative = ViewKey<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::from_str(&viewkey);
        match native_viewkey_result {
            Ok(v) => {
                native_viewkey = v;
            }
            Err(_) => {
                return (true, "Error: Could not parse view key".to_string());
            }
        }

        let native_address;
        let native_address_result = AddressNative::try_from(&native_viewkey);
        match native_address_result {
            Ok(v) => {
                native_address = v.to_string();
            }
            Err(_) => {
                return (
                    true,
                    "Error: Could not derive address from view key".to_string(),
                );
            }
        }

        return (false, native_address);
    } else {
        return (true, "Error: undefined behavior".to_string());
    }
}

#[tauri::command]
pub fn sign(
    _handle: tauri::AppHandle,
    network: String,
    privatekey: String,
    message: String,
) -> (bool, String) {
    if network == "Mainnet".to_string() {
        pub use snarkvm::console::network::MainnetV0 as CurrentNetwork;

        pub type SignatureNative = Signature<CurrentNetwork>;
        pub type PrivateKeyNative = PrivateKey<CurrentNetwork>;

        let message_bytes = message.as_bytes();

        let native_privatekey;
        let native_privatekey_result = PrivateKeyNative::from_str(&privatekey);
        match native_privatekey_result {
            Ok(v) => {
                native_privatekey = v;
            }
            Err(_) => {
                return (true, "Error: Could not parse private key".to_string());
            }
        }

        let native_signature;
        let native_signature_result = SignatureNative::sign_bytes(
            &native_privatekey,
            message_bytes,
            &mut StdRng::from_entropy(),
        );
        match native_signature_result {
            Ok(v) => {
                native_signature = v;
            }
            Err(_) => {
                return (true, "Error: Could not sign message".to_string());
            }
        }

        return (false, native_signature.to_string());
    } else if network == "Testnet".to_string() {
        pub use snarkvm::console::network::TestnetV0 as CurrentNetwork;

        pub type SignatureNative = Signature<CurrentNetwork>;
        pub type PrivateKeyNative = PrivateKey<CurrentNetwork>;

        let message_bytes = message.as_bytes();

        let native_privatekey;
        let native_privatekey_result = PrivateKeyNative::from_str(&privatekey);
        match native_privatekey_result {
            Ok(v) => {
                native_privatekey = v;
            }
            Err(_) => {
                return (true, "Error: Could not parse private key".to_string());
            }
        }

        let native_signature;
        let native_signature_result = SignatureNative::sign_bytes(
            &native_privatekey,
            message_bytes,
            &mut StdRng::from_entropy(),
        );
        match native_signature_result {
            Ok(v) => {
                native_signature = v;
            }
            Err(_) => {
                return (true, "Error: Could not sign message".to_string());
            }
        }

        return (false, native_signature.to_string());
    } else {
        return (true, "Error: undefined behavior".to_string());
    }
}

#[tauri::command]
pub fn verify(
    _handle: tauri::AppHandle,
    network: String,
    address: String,
    message: String,
    signature: String,
) -> (bool, String) {
    if network == "Mainnet".to_string() {
        pub use snarkvm::console::network::MainnetV0 as CurrentNetwork;

        pub type SignatureNative = Signature<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let message_bytes = message.as_bytes();

        let native_address;
        let native_address_result = AddressNative::from_str(&address);
        match native_address_result {
            Ok(v) => native_address = v,
            Err(_) => {
                return (true, "Error: Could not parse address".to_string());
            }
        }

        let native_signature;
        let native_signature_result = SignatureNative::from_str(&signature);
        match native_signature_result {
            Ok(v) => {
                native_signature = v;
            }
            Err(_) => {
                return (true, "Error: Could not sign message".to_string());
            }
        }

        let result = native_signature.verify_bytes(&native_address, message_bytes);

        match result {
            true => return (false, "Message verified successfully!".to_string()),
            false => return (true, "Message verification failed.".to_string()),
        }
    } else if network == "Testnet".to_string() {
        pub use snarkvm::console::network::TestnetV0 as CurrentNetwork;

        pub type SignatureNative = Signature<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let message_bytes = message.as_bytes();

        let native_address;
        let native_address_result = AddressNative::from_str(&address);
        match native_address_result {
            Ok(v) => native_address = v,
            Err(_) => {
                return (true, "Error: Could not parse address".to_string());
            }
        }

        let native_signature;
        let native_signature_result = SignatureNative::from_str(&signature);
        match native_signature_result {
            Ok(v) => {
                native_signature = v;
            }
            Err(_) => {
                return (true, "Error: Could not sign message".to_string());
            }
        }

        let result = native_signature.verify_bytes(&native_address, message_bytes);

        match result {
            true => return (false, "Message verified successfully!".to_string()),
            false => return (true, "Message verification failed.".to_string()),
        }
    } else {
        return (true, "Error: undefined behavior".to_string());
    }
}

#[tauri::command]
pub fn decrypt_record(
    _handle: tauri::AppHandle,
    network: String,
    ciphertext: String,
    viewkey: String,
) -> (bool, String) {
    if network == "Mainnet".to_string() {
        pub use snarkvm::console::network::MainnetV0 as CurrentNetwork;

        pub type CiphertextNative = Ciphertext<CurrentNetwork>;
        pub type RecordCiphertextNative = Record<CurrentNetwork, CiphertextNative>;
        pub type ViewKeyNative = ViewKey<CurrentNetwork>;

        let native_ciphertext;

        let native_ciphertext_result = RecordCiphertextNative::from_str(&ciphertext);
        match native_ciphertext_result {
            Ok(v) => {
                native_ciphertext = v;
            }
            Err(_) => {
                return (true, "Error: Could not parse ciphertext".to_string());
            }
        }

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::from_str(&viewkey);
        match native_viewkey_result {
            Ok(v) => {
                native_viewkey = v;
            }
            Err(_) => {
                return (true, "Error: Could not parse view key".to_string());
            }
        }

        let plaintext;
        let native_plaintext_result = native_ciphertext.decrypt(&native_viewkey);
        match native_plaintext_result {
            Ok(v) => {
                plaintext = v.to_string();
            }
            Err(_) => {
                return (
                    true,
                    "Error: Invalid view key for provided ciphertext".to_string(),
                );
            }
        }

        return (false, plaintext);
    } else if network == "Testnet".to_string() {
        pub use snarkvm::console::network::TestnetV0 as CurrentNetwork;

        pub type CiphertextNative = Ciphertext<CurrentNetwork>;
        pub type RecordCiphertextNative = Record<CurrentNetwork, CiphertextNative>;
        pub type ViewKeyNative = ViewKey<CurrentNetwork>;

        let native_ciphertext;

        let native_ciphertext_result = RecordCiphertextNative::from_str(&ciphertext);
        match native_ciphertext_result {
            Ok(v) => {
                native_ciphertext = v;
            }
            Err(_) => {
                return (true, "Error: Could not parse ciphertext".to_string());
            }
        }

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::from_str(&viewkey);
        match native_viewkey_result {
            Ok(v) => {
                native_viewkey = v;
            }
            Err(_) => {
                return (true, "Error: Could not parse view key".to_string());
            }
        }

        let plaintext;
        let native_plaintext_result = native_ciphertext.decrypt(&native_viewkey);
        match native_plaintext_result {
            Ok(v) => {
                plaintext = v.to_string();
            }
            Err(_) => {
                return (
                    true,
                    "Error: Invalid view key for provided ciphertext".to_string(),
                );
            }
        }

        return (false, plaintext);
    } else {
        return (true, "Error: undefined behavior".to_string());
    }
}




/// A helper function to recursively load the program and all of its imports into the process. Lifted from snarkOS.
fn load_program_from_network<N: Network>(
    process: &mut Process<N>,
    program_id: String,
    network: &str,
    endpoint: &str,
) -> Result<(), LeoError> {
    let (program, output) =  {
        let program = if program_id.ends_with(".aleo") {
            Package::is_aleo_name_valid(&program_id[0..program_id.len() - 5]);
            program_id
        } else {
            Package::is_aleo_name_valid(&program_id);
            format!("{}.aleo", program_id)
        };

        // Build custom url to fetch from based on the flags and user's input.
        let url = format!("program/{}", program);

        (program, url)
    };

    let url = format!("{}/{}/{output}", &endpoint, &network);
    let result = fetch_from_network(&url)?;

    // Verify that the source file parses into a valid Aleo program.
    verify_valid_program::<N>(&program, &result)?;
    
    // Fetch the program.
    let program_src = result;
    let program = SnarkVMProgram::<N>::from_str(&program_src)?;

    // Iterate through the program imports.
    for import_program_id in program.imports().keys() {
        // Add the imports to the process if does not exist yet.
        if !process.contains_program(import_program_id) {
            // Recursively load the program and its imports.
            load_program_from_network(process, import_program_id.to_string(), network, endpoint)?;
        }
    }

    // Add the program to the process if it does not already exist.
    if !process.contains_program(program.id()) {
        process.add_program(&program)?;
    }

    Ok(())
}

pub fn execute_remote <A: Aleo>(
    program_name : String,
    function_name : String,
    network: String,
    endpoint : String,
    private_key : String,
    inputs : Vec<String>,
) -> Result<Vec<String>, LeoError>{
    
    //Initialize an RNG
    let rng = &mut rand::thread_rng();

    let network_name : &str;
    if network == "Mainnet".to_string(){
        network_name  = "mainnet";
    } else {
        network_name  = "testnet";
    }

    // Initialize the storage.
    let store = ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(StorageMode::Production)?;

    // Initialize the VM.
    let vm = VM::from(store)?;

    // Remove the `.aleo` extension from the program name, if it exists.
    let program_name = match program_name.strip_suffix(".aleo") {
        Some(name) => name.to_string(),
        None => program_name,
    };
    // Load the main program, and all of its imports.
    let program_id = &ProgramID::<A::Network>::from_str(&format!("{program_name}.aleo"))?;
    load_program_from_network(&mut vm.process().write(), program_name, network_name, &endpoint)?;


    // Need to compute the authorization.
    let authorization = vm.authorize(&PrivateKey::from_str(&private_key).unwrap(), program_id, function_name, inputs, rng)?;
    // Execute the circuit.
    let (response, _) = vm.process().read().execute::<A, _>(authorization, rng)?;

    let mut results = Vec::new();

    for val in response.outputs(){
        match val {
            snarkvm::prelude::Value::Future(_) => {},
            _ => {results.push(val.to_string());}
        }
    }
    // let final_result = &results[results.len()-1];
    // println!("{}",final_result);
    Ok(results)
}

#[tauri::command]
pub async fn execute_remote_wrapper (
    _handle: tauri::AppHandle,
    program : String,
    function : String,
    network: String,
    endpoint : String,
    pk: String,
    inputs : Vec<String>,
) -> (bool, Vec<String>){

    if network == "Mainnet".to_string(){
        match execute_remote::<AleoV0>(program, function, network, endpoint, pk, inputs){
            Ok(results) => { return (false,results)},
            Err(_) => {return (true,Vec::new())}
        }
    } else {
        match execute_remote::<AleoTestnetV0>(program, function, network, endpoint, pk, inputs){
            Ok(results) => { return (false,results)},
            Err(_) => {return (true,Vec::new())}
        }
    }

}

// #[tauri::command]

// pub fn estimate_fee () {
//     pub async fn estimate_deployment_fee(program: &str, imports: Option<Object>) -> Result<u64, String> {
//         log(
//             "Disclaimer: Fee estimation is experimental and may not represent a correct estimate on any current or future network",
//         );
//         let mut process_native = ProcessNative::load_web().map_err(|err| err.to_string())?;
//         let process = &mut process_native;

//         log("Check program has a valid name");
//         let program = ProgramNative::from_str(program).map_err(|err| err.to_string())?;

//         log("Check program imports are valid and add them to the process");
//         ProgramManager::resolve_imports(process, &program, imports)?;

//         log("Create sample deployment");
//         let deployment =
//             process.deploy::<CurrentAleo, _>(&program, &mut StdRng::from_entropy()).map_err(|err| err.to_string())?;
//         if deployment.program().functions().is_empty() {
//             return Err("Attempted to create an empty transaction deployment".to_string());
//         }

//         let (minimum_deployment_cost, (_, _, _)) =
//             deployment_cost::<CurrentNetwork>(&deployment).map_err(|err| err.to_string())?;

//         Ok(minimum_deployment_cost)
//     }

// }