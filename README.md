<p align="center"> 
    <img alt="den" width="300" src="./src-tauri/icons/den.png">
</p>

# Den

Built with Tauri + Leptos


# TODO:
## General:
- Integrate Devnet support
- Work on Custom Titlebar with buttons
- Need to figure out way to integrate LSP server for document-wide error checking
- Migrate leo.exe stuff to underlying snarkVM code for platform agnostic support?
- Disable button clicking when backend/API requests are pending

## Sidebar:
- General:
    - Add hover names for sidebar icon buttons
    - Persistant state between closing and reopening the app
- File System:
    - Reset FS default directory
    - Add way to open new file directory
    - Add way to save files
    - Add check if user tries to close file without it being saved
    - Add hover filepaths for file buttons
    - Add way to delete files
    - Add screen if file cannot be found
- Account:
    - Create New Account
        - Add way to save accounts once generated
    - Load Account from PK
        - Add way to save accounts once loaded
    - Sign a Message
        - Add button functionality
        - Add conditional output section to card body
    - Verify a Message
        - Add button functionality
        - Add conditional output section to card body
- Records:
    - Add way to save records?
- Compile
    - Add this
- Deploy and Execute:
    - Add way to use private fees for deployment
    - Add deployed program interaction
- REST API:
    - Get Latest Block:
        - Add buttons to clear and open latest block once retrieved
    - Get Program:
        - Add buttons to clear and open program once retrieved
    - Else
        - Add functionality and output


## Editor
- Dynamic error checking (red squiggle with error description)
- Highlight current line of text with gray?
- Add highlighting conditional on file type (.leo vs .aleo vs. everything else)

## Terminal:
- Start work on this




