pub use snarkvm_console::{
    account::{Address, PrivateKey, Signature, ViewKey},
    program::{Ciphertext, Record},
};

use rand::{rngs::StdRng, SeedableRng};
use std::str::FromStr;

#[tauri::command]
pub fn new_account(_handle: tauri::AppHandle, network: String) -> (bool, (String, String, String)) {
    if network == "Mainnet".to_string() {
        pub use snarkvm_console::network::MainnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::TestnetV0 as CurrentNetwork;
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
        pub use snarkvm_console::network::MainnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::TestnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::MainnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::TestnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::MainnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::TestnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::MainnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::TestnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::MainnetV0 as CurrentNetwork;

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
        pub use snarkvm_console::network::TestnetV0 as CurrentNetwork;

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
