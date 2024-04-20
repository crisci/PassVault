pub mod account {

    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    use serde::{Deserialize, Serialize};

    use crate::utils::encrypt_data;

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    ) -> Result<(), String> {
        let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
        let new_dir = PathBuf::from(format!(
            "{}/{}",
            dir.data_local_dir()
                .to_str()
                .ok_or("Error getting data local dir")?,
            "PassVault"
        ));
        let file_path = new_dir.join("accounts.config");

        if !new_dir.exists() {
            fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
        }
        let mut file =
            File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
        let serialized = serde_json::to_string(accounts)
            .map_err(|err| format!("Serialization error: {}", err))?;
        println!("{}", serialized);
        let serialized_enc = encrypt_data(&serialized, symmetric_key).unwrap();
        file.write_all(serialized_enc.as_bytes())
            .map_err(|err| format!("Error writing to file: {}", err))?;
        Ok(())
    }
}
