use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf, str::FromStr,
};

use openssl::{
   rand, rsa::{Padding, Rsa}, symm::{encrypt, Cipher}
};
use rfd::FileDialog;


pub mod utils {



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

pub fn generate_key_pair(sk_path: String) {
    let key_pair = Rsa::generate(2048).expect("Error generating the keys");
    let secret_key = 
        key_pair
            .private_key_to_pem()
            .expect("Error extracting the pk");

    let mut aes_key = [0; 32];
    rand::rand_bytes(&mut aes_key).unwrap();

    let mut buf = vec![0; key_pair.size() as usize];

    let _key_enc = key_pair.public_encrypt(&aes_key, &mut buf, Padding::PKCS1).unwrap();
    let key_enc_hex = hex::encode(buf);

    let sk_enc = encrypt_data(&String::from_utf8(secret_key).unwrap(), &aes_key).unwrap();
    let sk_enc_b64 = hex::encode(sk_enc);


    match locate_keys(key_enc_hex, sk_enc_b64, sk_path) {
        Ok(_) => println!("Keys located"),
        Err(e) => println!("{}", e),
    }
}

fn locate_keys(symm_key: String, sk: String, sk_path: String) -> Result<(), String> {
    let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
    let new_dir = PathBuf::from(format!(
        "{}/{}",
        dir.data_local_dir()
            .to_str()
            .ok_or("Error getting data local dir")?,
        "PassVault"
    ));
    let sk_path = PathBuf::from_str(&format!("{}/secret_key.pem", sk_path)).unwrap();
    let symm = new_dir.join("symm.key");

    println!("{:?}", new_dir);

    if !symm.exists() {
        fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
        // First time creation
        let mut sk_file = File::create(&sk_path).map_err(|err| format!("Error creating sk file: {}", err))?;
        let mut symm_file = File::create(&symm).map_err(|err| format!("Error creating pk file: {}", err))?;

        sk_file.write_all(sk.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
        symm_file.write_all(symm_key.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;

        println!("key created at {}", new_dir.to_str().unwrap());
        
    }

    Ok(())
}


pub fn encrypt_data(data: &String, symmetric_key: &[u8]) -> Result<String, String> {

        let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

        let aes = Cipher::aes_256_cbc();
        let data_enc = encrypt(aes, symmetric_key, Some(iv), data.as_bytes()).expect("Unable to decrypt the key");
        let data_enc_hex = hex::encode(data_enc);
        
        return Ok(data_enc_hex);
}


pub fn is_key_created() -> bool {
    let dir = match directories::BaseDirs::new() {
        Some(dir) => dir,
        None => return false
    };
    let dir_str = match dir.data_local_dir().to_str() {
        Some(s) => s,
        None => return false,
    };

    let keys_dir = PathBuf::from(format!(
        "{}/{}",
        dir_str,
        "PassVault"
    ));
    let pk_path = keys_dir.join("symm.key");
    pk_path.exists()
}

pub fn select_path() -> Option<String>{
    let dir = directories::UserDirs::new().unwrap();
    let home_dir = dir.home_dir();
    let result = FileDialog::new().set_directory(home_dir).pick_folder();
    match result {
        Some(selected_path) => {Some(selected_path.into_os_string().into_string().unwrap())},
        None => {None}
    }
}