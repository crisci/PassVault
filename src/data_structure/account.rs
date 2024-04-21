pub mod account {

    use std::{
        fs::{self, File},
        io::Write,
    };

    use anyhow::Context;
    use serde::{Deserialize, Serialize};

    use crate::{enums::error::error::CryptoError, utils::{decrypt_data, encrypt_data}};

    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Account {
        host: String,
        username: String,
        key: String,
    }
    
    impl Account {
        pub fn new(host: String, username: String, key: String) -> Self {
            Self {
                host,
                username,
                key,
            }
        }

        pub fn get_host(&self) -> &String {
            &self.host
        }

        pub fn get_username(&self) -> &String {
            &self.username
        }
        pub fn get_key(&self) -> &String {
            &self.key
        }

        pub fn set_host(&mut self, host: String) {
            self.host = host
        }

        pub fn set_username(&mut self, username: String) {
            self.username = username
        }

        pub fn set_key(&mut self, key: String) {
            self.key = key
        }
    }

    pub fn serialize_accounts(
        accounts: &Vec<Account>,
        symmetric_key: &[u8],
    ) -> anyhow::Result<()> {
        let dir = directories::BaseDirs::new().ok_or(CryptoError::Unknown)?;
        let passvault_dir = dir.data_local_dir().join("PassVault");
        let account_file_path = passvault_dir.join("accounts.dat");

        if !passvault_dir.exists() {
            fs::create_dir_all(&passvault_dir)?;
        }
        let mut account_file = File::create(&account_file_path)?;
        let serialized = serde_json::to_string(accounts)?;
        let serialized_enc = encrypt_data(&serialized, symmetric_key).unwrap();
        account_file.write_all(serialized_enc.as_bytes())?;
        Ok(())
    }

    pub fn deserialize_accounts(
        symmetric_key: &[u8],
    ) -> anyhow::Result<Vec<Account>> {
        let dir = directories::BaseDirs::new().context(CryptoError::Unknown)?;
        let passvault_dir = dir.data_local_dir().join("PassVault");
        let account_file_path = passvault_dir.join("accounts.dat");

        if !account_file_path.exists() {
            return Ok(Vec::new());
        }

        let serialized_enc = fs::read(account_file_path)?;
        if serialized_enc.is_empty() {
            return Ok(Vec::new());
        }
        let serialized = String::from_utf8(serialized_enc)?;
        let deserialized = decrypt_data(&serialized, symmetric_key)?;
        let accounts: Vec<Account> = serde_json::from_str(&deserialized)?;
        Ok(accounts)
    }
}
