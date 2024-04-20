use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf, str::FromStr,
};

use directories::BaseDirs;
use openssl::{
   rand, rsa::{Padding, Rsa}, symm::{decrypt, encrypt, Cipher}
};
use rfd::FileDialog;


pub mod utils {
    use std::{fs::File, path::PathBuf};


    pub fn create_passvault_files() {
        let dir = match directories::BaseDirs::new() {
            Some(dir) => dir,
            None => return
        };
        let dir_str = match dir.data_local_dir().to_str() {
            Some(s) => s,
            None => return,
        };

        let passvault_path = PathBuf::from(format!(
            "{}/{}",
            dir_str,
            "PassVault"
        ));
        let config_path = passvault_path.join("data.config");
        let acccounts_path = passvault_path.join("accounts.dat");
        File::create(&acccounts_path).expect("Error creating accounts file");
        File::create(&config_path).expect("Error creating config file");
    }


    pub fn _pad16(s: &String) -> String {
        format!("{:0<16}", s)
    }

    pub fn _pad32(s: &String) -> String {
        format!("{:0<32}", s)
    }

    pub fn _pad512(s: &String) -> String {
        format!("{:-<512}", s)
    }

}

pub fn generate_key_pair(aes_path: String, password: String) -> Vec<u8> {

    let pwd_32 = utils::_pad32(&password);
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let aes = Cipher::aes_256_cbc();
    let mut aes_key = [0; 32];
    rand::rand_bytes(&mut aes_key).unwrap();

    println!("aes_key: {:?}", aes_key);

    let aes_key_enc = encrypt(aes, pwd_32.as_bytes(), Some(iv), &aes_key);
    let aes_key_enc_hex = hex::encode(aes_key_enc.unwrap());

    match locate_keys(aes_key_enc_hex, aes_path) {
        Ok(_) => println!("Keys located"),
        Err(e) => println!("Error locating keys: {}", e),
    }

    aes_key.to_vec()

}

fn locate_keys(aes_key: String, aes_path: String) -> Result<(), String> {
    let aes_dir = PathBuf::from(aes_path);
    let aes_file_path = aes_dir.join("aes.dat");

    if !aes_file_path.exists() {
        // First time creation
        let mut file = File::create(&aes_file_path).map_err(|err| format!("Error creating aes file: {}", err))?;
        file.write_all(aes_key.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
        println!("key created at {}", aes_file_path.to_str().unwrap());
    }

    Ok(())
}

pub fn check_decryption_key(aes_path: String, password: String) -> Result<Vec<u8>, String> {
    let pwd_32 = utils::_pad32(&password);
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let aes = Cipher::aes_256_cbc();
    let aes_key_enc_b64 = fs::read(aes_path).map_err(|err| format!("Error reading aes file: {}", err))?;
    let aes_key_enc = hex::decode(aes_key_enc_b64).unwrap();
    let aes_key = decrypt(aes, pwd_32.as_bytes(), Some(iv), &aes_key_enc).map_err(|err| format!("Error decrypting aes key: {}", err))?;

    Ok(aes_key)
}


pub fn encrypt_data(data: &String, symmetric_key: &[u8]) -> Result<String, String> {

        let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

        let aes = Cipher::aes_256_cbc();
        let data_enc = encrypt(aes, symmetric_key, Some(iv), data.as_bytes()).expect("Unable to decrypt the key");
        let data_enc_hex = hex::encode(data_enc);
        
        return Ok(data_enc_hex);
}

pub fn decrypt_data(data_enc_hex: &String, symmetric_key: &[u8]) -> Result<String, String> {

    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    
    let aes = Cipher::aes_256_cbc();
    
    let data_enc = hex::decode(data_enc_hex).unwrap();
    println!("symmetric_key: {:?}", symmetric_key);
    let data = decrypt(aes, symmetric_key, Some(iv), &data_enc).expect("Unable to decrypt the key");
    
    let data_str = String::from_utf8(data).expect("Unable to convert to string");
    
    Ok(data_str)
}


pub fn is_key_created() -> bool {
    let base_dir = BaseDirs::new().unwrap();
    let passvault_path = base_dir.data_local_dir().join("PassVault");
    let config_path = passvault_path.join("data.config");
    let accounts_path = passvault_path.join("accounts.dat");
    config_path.exists() || accounts_path.exists()
}

pub fn select_path() -> Option<String>{
    let dir = directories::UserDirs::new().unwrap();
    let home_dir = dir.home_dir();
    let result = match is_key_created() {
        true => FileDialog::new().set_directory(home_dir).pick_file(),
        false => FileDialog::new().set_directory(home_dir).pick_folder()
    };
    
    match result {
        Some(selected_path) => Some(selected_path.into_os_string().into_string().unwrap()),
        None => None,
    }
}

pub fn read_aes_key(aes_path: String) -> Result<Vec<u8>, String> {
    let aes_key = fs::read(aes_path).map_err(|err| format!("Error reading aes file: {}", err))?;
    Ok(aes_key)
}