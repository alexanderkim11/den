
pub use snarkvm_console::{
    account::{Address, Signature, ViewKey},
    program::{
        Ciphertext,
        Record,
    }
};

use std::str::FromStr;

#[tauri::command]
pub fn decrypt_record(_handle: tauri::AppHandle, network : String, ciphertext: String, viewkey : String) -> (bool,String){
    if network == "Mainnet".to_string(){
        pub use snarkvm_console::network::MainnetV0 as CurrentNetwork;

        pub type CiphertextNative = Ciphertext<CurrentNetwork>;
        pub type RecordCiphertextNative = Record<CurrentNetwork, CiphertextNative>;
        pub type ViewKeyNative = ViewKey<CurrentNetwork>;

        let native_ciphertext;

        let native_ciphertext_result  = RecordCiphertextNative::from_str(&ciphertext);
        match native_ciphertext_result {
            Ok(v) =>{
                native_ciphertext = v;
            },
            Err(_) => {
                return (true,"Error: Could not parse ciphertext".to_string());
            }
        }

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::from_str(&viewkey);
        match native_viewkey_result {
            Ok(v) =>{
                native_viewkey = v;
            },
            Err(_) => {
                return (true,"Error: Could not parse view key".to_string());
            }
        }

        let plaintext;
        let native_plaintext_result = native_ciphertext.decrypt(&native_viewkey);
        match native_plaintext_result {
            Ok(v) =>{
                plaintext = v.to_string();
            },
            Err(_) => {
                return (true, "Error: Invalid view key for provided ciphertext".to_string());
            }
        }        

        return (false,plaintext);

    } else if network == "Testnet".to_string() {
        pub use snarkvm_console::network::TestnetV0 as CurrentNetwork;

        pub type CiphertextNative = Ciphertext<CurrentNetwork>;
        pub type RecordCiphertextNative = Record<CurrentNetwork, CiphertextNative>;
        pub type ViewKeyNative = ViewKey<CurrentNetwork>;

        let native_ciphertext;

        let native_ciphertext_result  = RecordCiphertextNative::from_str(&ciphertext);
        match native_ciphertext_result {
            Ok(v) =>{
                native_ciphertext = v;
            },
            Err(_) => {
                return (true,"Error: Could not parse ciphertext".to_string());
            }
        }

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::from_str(&viewkey);
        match native_viewkey_result {
            Ok(v) =>{
                native_viewkey = v;
            },
            Err(_) => {
                return (true,"Error: Could not parse view key".to_string());
            }
        }

        let plaintext;
        let native_plaintext_result = native_ciphertext.decrypt(&native_viewkey);
        match native_plaintext_result {
            Ok(v) =>{
                plaintext = v.to_string();
            },
            Err(_) => {
                return (true, "Error: Invalid view key for provided ciphertext".to_string());
            }
        }        

        return (false,plaintext);
    } else {
        return (true, "Error: undefined behavior".to_string());
    }

}

#[tauri::command]
pub fn address_from_vk(_handle: tauri::AppHandle, network : String, viewkey : String) -> (bool,String){
    if network == "Mainnet".to_string(){
        pub use snarkvm_console::network::MainnetV0 as CurrentNetwork;

        pub type ViewKeyNative = ViewKey<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::from_str(&viewkey);
        match native_viewkey_result {
            Ok(v) =>{
                native_viewkey = v;
            },
            Err(_) => {
                return (true,"Error: Could not parse view key".to_string());
            }
        }

        let native_address;
        let native_address_result = AddressNative::try_from(&native_viewkey);
        match native_address_result {
            Ok(v) =>{
                native_address = v.to_string();
            },
            Err(_) => {
                return (true,"Error: Could not derive address from view key".to_string());
            }
        }

        return (false,native_address);

    } else if network == "Testnet".to_string() {

        pub use snarkvm_console::network::TestnetV0 as CurrentNetwork;

        pub type ViewKeyNative = ViewKey<CurrentNetwork>;
        pub type AddressNative = Address<CurrentNetwork>;

        let native_viewkey;
        let native_viewkey_result = ViewKeyNative::from_str(&viewkey);
        match native_viewkey_result {
            Ok(v) =>{
                native_viewkey = v;
            },
            Err(_) => {
                return (true,"Error: Could not parse view key".to_string());
            }
        }

        let native_address;
        let native_address_result = AddressNative::try_from(&native_viewkey);
        match native_address_result {
            Ok(v) =>{
                native_address = v.to_string();
            },
            Err(_) => {
                return (true,"Error: Could not derive address from view key".to_string());
            }
        }

        return (false,native_address);
    } else {
        return (true, "Error: undefined behavior".to_string());
    }

}



pub fn verify(_handle: tauri::AppHandle, network : String, ciphertext: String, viewkey : String){}

pub fn sign(_handle: tauri::AppHandle, network : String, ciphertext: String, viewkey : String){}
