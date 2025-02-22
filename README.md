<p align="center"> 
    <img alt="den" width="300" src="./src-tauri/icons/den.png">
</p>

# Den

Built with Tauri + Leptos

# TODO:

**BOLD** signifies priority tasking

## General:
- Change blocking backend operations to non-blocking?
- **Integrate Devnet support**
- Work on Custom Titlebar with buttons
- Need to figure out way to integrate LSP server for document-wide error checking
- Testing framework?
- Add way to open multiple windows/app instances

## Sidebar:
- General:
    - Add hover names for sidebar icon buttons?
    - Persistant state between closing and reopening the app
        - **Update state filepath**
        - Persistant state is challenging due to little support for Leptos + Tauri combo
        - Need to save state each time any of the items change
        - State should be:
            - Session/directory name
                - **open_files**
                - **selected_file**
                - **cached_files**
                - **saved_files**
                - accounts
                - open_directory
- File System:
    - **Reset FS default directory**
        - Check for unsaved open files -> send warning if true
        - Reload window with new filesystem open
    - Add hover filepaths for file buttons
    - **Add way to create/delete files and directories**
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
    - Add sub-icon on sidebar icon to indicator success,failure, or pending
    - **Ctrl + S on IDE should automatically compile a Leo program**
- **Deploy and Execute:**
    - Add indicator for current network
    - Add functionality here
    - Add deployed program interaction
    - Figure out how to migrate leo.exe stuff to underlying snarkVM code for platform agnostic support?
- REST API:
    - Figure out how to migrate leo.exe stuff to underlying snarkVM code for platform agnostic support?
    - Reset all output fields when environment is changed
    - Get Account Balance:
        - Is there a better way to do this other than brute-force searching for record?
        - **Add functionality and output**

## Editor
- Dynamic error checking (red squiggle with error description) (See LSP task above)
- Better file state tracking (Ctrl + Z Undo doesn't work once you switch to another file)
- Highlight current line of text with gray?
- **Add highlighting conditional on file type (.leo vs .aleo vs. everything else)**
- **Maintain scroll level and cursor position when switching between file tabs and reopening files**

## Terminal:
- **Start work on this**




