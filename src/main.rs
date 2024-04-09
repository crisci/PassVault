
use std::{fs::{self, File}, io::{BufReader, Read, Write}, path::PathBuf};

use enums::{KeyStatus};
use openssl::{error::Error, pkey::Private, rsa::Rsa};
use rand::{thread_rng, Rng};
use base64::{engine::general_purpose, Engine};

mod enums;

fn main() {
    let mut key = [0;64];
    thread_rng().try_fill(&mut key).expect("Error to create the key");
    let key_pair = Rsa::generate(2048).expect("Error generating the keys");
    let private_key = private_key(key_pair).unwrap().extract();
    if let Some(bytes) = private_key {
        println!("{:?}", bytes);
    }
}


fn private_key(key_pair: Rsa<Private>) -> Result<KeyStatus, String>{
    let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
    let new_dir = PathBuf::from(format!("{}/{}", dir.data_local_dir().to_str().ok_or("Error getting data local dir")?, "PassVault"));
    let file_path = new_dir.join("private_key.pem");
    let private_key = key_pair.private_key_to_pem().expect("Error private key");

    if !new_dir.exists() {
        fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
        // First time creation
        let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
        file.write_all(&private_key).map_err(|err| format!("Error writing to file: {}", err))?;
        return Ok(KeyStatus::CREATED(private_key));
    } else {
        // File already exists, so read the file
        let mut key = Vec::new();
        let file = File::open(&file_path).map_err(|err| format!("Error opening file: {}", err))?;
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut key).map_err(|err| format!("Errore nella lettura del file: {}", err))?;
        return Ok(KeyStatus::PRESENT(private_key));
    }
}

fn public_key(key_pair: Rsa<Private>) -> Result<KeyStatus, String>{
    let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
    let new_dir = PathBuf::from(format!("{}/{}", dir.data_local_dir().to_str().ok_or("Error getting data local dir")?, "PassVault"));
    let file_path = new_dir.join("private_key.pem");
    let key_pair = Rsa::generate(2048).expect("Error generating the keys");
    let private_key = key_pair.private_key_to_pem().expect("Error private key");

    if !new_dir.exists() {
        fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
        // First time creation
        let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
        file.write_all(&private_key).map_err(|err| format!("Error writing to file: {}", err))?;
        return Ok(KeyStatus::CREATED(private_key));
    } else {
        // File already exists, so read the file
        let mut key = Vec::new();
        let file = File::open(&file_path).map_err(|err| format!("Error opening file: {}", err))?;
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut key).map_err(|err| format!("Errore nella lettura del file: {}", err))?;
        return Ok(KeyStatus::PRESENT(private_key));
    }
}