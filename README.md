<p align="center"> 
    <img alt="den" width="300" src="./src-tauri/icons/den.png">
</p>

# Den

Built with Tauri + Leptos

# RELEASE TODOS:
    1. Change state path to "./"
    2. Replace all Rust `.unwrap()` calls with `.expect(ERROR_MESSAGE)`

# TODO:

**BOLD** signifies priority tasking

## General:
- **Work on Custom Titlebar with buttons**
- **Need to figure out way to use LSP server rather than hand-rolled nonsense**
- Testing framework?
- Add way to open multiple windows/app instances
- Figure out way to add warning message when exiting app
- Integrate Git

## Sidebar:
- General:
    - Add hover names for sidebar icon buttons?
    - Persistant state between closing and reopening the app
        - Persistant state is challenging due to little support for Leptos + Tauri combo
        - Need to save state each time any of the items change
        - State should be:
            - Session/directory name
                - TODO: open_files
                - TODO: selected_file
                - TODO: cached_files
                - TODO: saved_files
                - accounts
                - open_directory
- File System:
    - **Right Click Pop-up Menu**
        - **Delete**
        - **Copy**
        - **Rename**
        - Open File
        - Open Folder
        - Copy Path
    - Changing directories
        - Check for unsaved open files and only send warning if true
        - Reload window with new filesystem open?
    - Add hover filepaths for file buttons
    - Creating files/directories
        - Add better file/dir name parsing to check for invalid names
            - Add better error messages for invalid names
        - Add better auto sorting to FS when files/directories are created

- Environment:
    - Add more features?
    - Add way to edit API endpoint
- Account
    - Add authentication/authorization for accounts when either loading from persistant state or creating transactions?
    - Add background async function to periodically scan and update saved account balances, also should run whenever transactions are sent out
    - Saved Accounts
        - **Add field showing account balance**
        - Hide PK and VK by default (password protected?)
- Records:
    - Add way to save records?
- Compile
    - Add sub-icon on sidebar icon to indicate success,failure, or pending
- Deploy and Execute:
    - **Add message if broadcasting fails**
    - Deploy Program
        - **Add fee estimation functionality**
        - **Need better way to check if transaction fails due to low balance or fee not being large enough or conflicting program name**
    - Deployed/Loaded Program:
        - Figure out how compressed and expanded inputs are connected/separated and rest when function is switched between compressed and expanded?
    - Figure out how to migrate leo.exe stuff to underlying snarkVM code for platform agnostic support?
- REST API:
    - Figure out how to migrate leo.exe stuff to underlying snarkVM code for platform agnostic support?
    - Reset all output fields when environment is changed
    - Get Account Balance:
        - Private Balance: Is there a better way to do this other than brute-force searching for record?

- **Transaction History:**
    - **add this**

## Editor
- Dynamic error checking (red squiggle with error description) (See LSP task above)
- Better file state tracking (Ctrl + Z Undo doesn't work once you switch to another file)
- Highlight current line of text with gray?
- **Maintain scroll level and cursor position when switching between file tabs and reopening files**

## Terminal:
- **Start work on this**




