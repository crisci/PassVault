use std::{io::Read, ops::Deref, panic};

use openssl::{aes::{self, aes_ige, AesKey}, cipher::Cipher, encrypt, rsa::Rsa, symm::{Crypter, Mode}};

use crate::utils::utils::{pad32, pad512, public_key};

pub mod utils {
    use std::{
        fs::{self, File},
        io::{BufReader, Read, Write},
        path::PathBuf,
    };

    use openssl::{pkey::Private, rsa::Rsa};

    use crate::enums::KeyStatus;

    pub fn pad16(s: &String) -> String { 
        format!("{:0<16}", s)
    }

    pub fn pad32(s: &String) -> String {
        format!("{:0<32}", s)
    }

    pub fn pad512(s: &String) -> String {
        format!("{:-<512}", s)
    }

    pub fn private_key(sk: Vec<u8>) -> Result<KeyStatus, String> {
        let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
        let new_dir = PathBuf::from(format!(
            "{}/{}",
            dir.data_local_dir()
                .to_str()
                .ok_or("Error getting data local dir")?,
            "PassVault"
        ));
        let file_path = new_dir.join("private_key.pem");

        if new_dir.exists() {
            fs::create_dir_all(&new_dir)
                .map_err(|err| format!("Error creating directory: {}", err))?;
            // First time creation
            let mut file =
                File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
            file.write_all(&sk)
                .map_err(|err| format!("Error writing to file: {}", err))?;
            return Ok(KeyStatus::CREATED(sk));
        } else {
            // File already exists, so read the file
            let mut key = Vec::new();
            let file =
                File::open(&file_path).map_err(|err| format!("Error opening file: {}", err))?;
            let mut reader = BufReader::new(file);
            reader
                .read_to_end(&mut key)
                .map_err(|err| format!("Errore nella lettura del file: {}", err))?;
            return Ok(KeyStatus::PRESENT(sk));
        }
    }

    pub fn public_key(pk: Vec<u8>) -> Result<KeyStatus, String> {
        let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
        let new_dir = PathBuf::from(format!(
            "{}/{}",
            dir.data_local_dir()
                .to_str()
                .ok_or("Error getting data local dir")?,
            "PassVault"
        ));
        let file_path = new_dir.join("public_key.pem");

        if new_dir.exists() {
            fs::create_dir_all(&new_dir)
                .map_err(|err| format!("Error creating directory: {}", err))?;
            // First time creation
            let mut file =
                File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
            file.write_all(&pk)
                .map_err(|err| format!("Error writing to file: {}", err))?;
            return Ok(KeyStatus::CREATED(pk));
        } else {
            // File already exists, so read the file
            let mut key = Vec::new();
            let file =
                File::open(&file_path).map_err(|err| format!("Error opening file: {}", err))?;
            let mut reader = BufReader::new(file);
            reader
                .read_to_end(&mut key)
                .map_err(|err| format!("Errore nella lettura del file: {}", err))?;
            return Ok(KeyStatus::PRESENT(pk));
        }
    }
}

pub fn generate_key_pair(symmetric_key: String) {
    let key_pair = Rsa::generate(2048).expect("Error generating the keys");
    let (private_key, public_key) = (key_pair.private_key_to_pem().expect("Error extracting the pk"), key_pair.public_key_to_pem().expect("Error extracting the sk"));

    let size = key_pair.size();

    let mut iv = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\
                \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";

    let plain = b"\x12\x34\x56\x78\x90\x12\x34\x56\x12\x34\x56\x78\x90\x12\x34\x56";
    let key = AesKey::new_encrypt(pad32(&symmetric_key).as_bytes()).unwrap();
    println!("{}", public_key.len());
    let mut pk_enc = [0u8; 512];
    let binding = pad512(&String::from_utf8(public_key).unwrap());
    let pad = binding.as_bytes();
    println!("{}-{}", pad.len(), pk_enc.len());
    aes_ige(pad, &mut pk_enc, &key, &mut iv, Mode::Encrypt);
    

    println!("DONE")
}
