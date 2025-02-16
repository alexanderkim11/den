<p align="center"> 
    <img alt="den" width="300" src="./src-tauri/icons/den.png">
</p>

# Den

Built with Tauri + Leptos

# TODO:

**BOLD** signifies priority tasking

## General:
- **Integrate Devnet support**
- Work on Custom Titlebar with buttons
- Need to figure out way to integrate LSP server for document-wide error checking
- Disable button clicking when backend/API requests are pending
- Testing framework?

## Sidebar:
- General:
    - Add hover names for sidebar icon buttons?
    - **Persistant state between closing and reopening the app**
    - Disable copy button if target field is empty
- File System:
    - **Reset FS default directory**
    - **Add way to open new file directory**
    - Add hover filepaths for file buttons
    - **Add way to delete files**
    - Add screen if file cannot be found

- Environment:
    - Add more features?
    - Add way to edit API endpoint
- Account
    - Add authentication/authorization for accounts when either loading from persistant state or creating transactions?
    - Add background async function to periodically scan and update saved account balances, also should run whenever transactions are sent out
    - Saved Accounts
        - Add fields showing account details once selected (Address, Balance, PK, VK, etc.)
        - Hide PK and VK by default (password protected?)
    - Create New Account
        - **Add error checks if user tries to save account with existing name**
        - Add error check if user tries to save the same (pk,vk,address) twice under different names?
    - Load Account from PK
        - **Add error checks if user tries to save account with existing name**
        - Add error check if user tries to save the same (pk,vk,address) twice under different names?
- Records:
    - Add way to save records?
- Compile
    - Add this?
- **Deploy and Execute:**
    - Add indicator for current network
    - Add functionality here
    - Add deployed program interaction
    - Migrate leo.exe stuff to underlying snarkVM code for platform agnostic support
- REST API:
    - **Figure out how to migrate leo.exe stuff to underlying snarkVM code for platform agnostic support?**
    - Get Latest Block:
        - Add button to open latest block once retrieved
    - Get Block by Height:
        - Add button to open block once retrieved
    - Get Program:
        - Add button to open program once retrieved
    - Get Transaction:
        - Add button to open transaction once retrieved
    - Get Account Balance:
        - Is there a better way to do this other than brute-force searching for record?
        - **Add functionality and output**

## Editor
- Dynamic error checking (red squiggle with error description) (See LSP task above)
- Highlight current line of text with gray?
- **Add highlighting conditional on file type (.leo vs .aleo vs. everything else)**
- **Maintain scroll level and cursor position when switching between file tabs and reopening files**
- Add unsaved changes indiciator to file tab

## Terminal:
- **Start work on this**




