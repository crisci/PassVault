pub mod config {
    use std::{fs::{self, File}, io::Write, path::PathBuf};

    use serde::{Deserialize, Serialize};


    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Config {
        sk_path: String,
    }

    impl Config {
        pub fn new(sk_path: String) -> Self {
            Self {
                sk_path,
            }
        }

        pub fn get_sk_path(&self) -> &String {
            &self.sk_path
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
                let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
                let config = Config::new("".to_string());
                let serialized = serde_json::to_string(&config).map_err(|err| format!("Serialization error: {}", err))?;
                file.write_all(serialized.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
                return Ok(config);
            } else {
                let file = File::open(&file_path).map_err(|err| format!("Error opening file: {}", err))?;
                let reader = std::io::BufReader::new(file);
                let config: Config = serde_json::from_reader(reader).map_err(|err| format!("Deserialization error: {}", err))?;
                return Ok(config);
            }
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
            return Ok(());
        }
    }
}
