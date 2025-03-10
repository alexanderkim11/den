<p align="center"> 
    <img alt="den" width="300" src="./src-tauri/icons/den.png">
</p>

# Den

Built with Tauri + Leptos

# RELEASE TODOS:
    1. Change state path to "./"

# TODO:

**BOLD** signifies priority tasking

## General:
- **Work on Custom Titlebar with buttons**
- Need to figure out way to integrate LSP server for document-wide error checking
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
    - **Add way to delete files**
    - Changing directories
        - **Check for unsaved open files -> send warning if true**
        - **Close all open files**
        - Reload window with new filesystem open?
    - Add hover filepaths for file buttons
    - Creating files/directories
        - **Add check for conflicting file and directory names**
        - Add better auto sorting to FS when files/directories are created
    - **Add screen if file cannot be found**
        - **Handle in FileTabs as Well**

- Environment:
    - Add more features?
    - Add way to edit API endpoint
- Account
    - Add authentication/authorization for accounts when either loading from persistant state or creating transactions?
    - Add background async function to periodically scan and update saved account balances, also should run whenever transactions are sent out
    - Saved Accounts
        - Add field showing account balance
        - Hide PK and VK by default (password protected?)
- Records:
    - Add way to save records?
- Compile
    - Add sub-icon on sidebar icon to indicate success,failure, or pending
- Deploy and Execute:
    - **Reset Fee and private fee Fields when deploy or function is called (prevents users from accidently spending something they don't want to)**
    - Deploy Program
        - **Add fee estimation functionality**
        - **Check if program has already been deployed on this network**
        - **Need better way to check if transaction fails due to low balance or fee not being large enough**
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
- **Add highlighting conditional on file type (.leo vs .aleo vs. everything else)**
- **Maintain scroll level and cursor position when switching between file tabs and reopening files**
- **Maintain indent level when entering newline character (VSCode-like)**

## Terminal:
- **Start work on this**




