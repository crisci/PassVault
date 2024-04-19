pub mod config {
    use std::{fs::{self, File}, io::Write, path::PathBuf};

    use serde::{Deserialize, Serialize};


    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Config {
        pk_path: String,
        sk_path: String,
    }

    impl Config {
        pub fn new(pk_path: String, sk_path: String) -> Self {
            Self { pk_path, sk_path }
        }

        pub fn get_pk_path(&self) -> &String {
            &self.pk_path
        }

        pub fn get_sk_path(&self) -> &String {
            &self.sk_path
        }

        pub fn set_pk_path(&mut self, pk_path: String) {
            self.pk_path = pk_path
        }

        pub fn set_sk_path(&mut self, sk_path: String) {
            self.sk_path = sk_path
        }

        pub fn get_config() -> Result<Self, String> {
            let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
            let new_dir = PathBuf::from(format!("{}/{}", dir.data_local_dir().to_str().ok_or("Error getting data local dir")?, "PassVault"));
            let file_path = new_dir.join("config.config");
            if !new_dir.exists() {
                fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
            }
            if !file_path.exists() {
                let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
                let config = Config::new(String::from(""), String::from(""));
                let serialized = serde_json::to_string(&config).map_err(|err| format!("Serialization error: {}", err))?;
                file.write_all(serialized.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
            }
            let file = File::open(&file_path).map_err(|err| format!("Error opening file: {}", err))?;
            let config: Config = serde_json::from_reader(file).map_err(|err| format!("Error reading file: {}", err))?;
            Ok(config)
        }

        pub fn save_config(self) -> Result<(), String> {
            let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
            let new_dir = PathBuf::from(format!("{}/{}", dir.data_local_dir().to_str().ok_or("Error getting data local dir")?, "PassVault"));
            let file_path = new_dir.join("config.config");
            if !new_dir.exists() {
                fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
            }
            let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
            let serialized = serde_json::to_string(&self).map_err(|err| format!("Serialization error: {}", err))?;
            file.write_all(serialized.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
            Ok(())
        }
    }
}
