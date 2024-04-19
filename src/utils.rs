use std::{
    fs::{self, File},
    io::{BufReader, Error, Read, Write},
    ops::Deref,
    panic,
    path::PathBuf,
};

use openssl::{
    aes::{self, aes_ige, AesKey},
    encrypt,
    rsa::{Padding, Rsa},
    symm::{decrypt, encrypt, Cipher, Crypter, Mode},
};

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

        if !new_dir.exists() {
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

        if !new_dir.exists() {
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

pub fn generate_key_pair(symmetric_key: &String) {
    let key_pair = Rsa::generate(2048).expect("Error generating the keys");
    let (secret_key, public_key) = (
        key_pair
            .private_key_to_pem()
            .expect("Error extracting the pk"),
        key_pair
            .public_key_to_pem()
            .expect("Error extracting the sk"),
    );

    let my_key_32 = format!("{:0>32}", symmetric_key);

    let key = my_key_32.as_bytes();
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let aes = Cipher::aes_256_cbc();
    let pk_enc_b64 = hex::encode(
        encrypt(aes, key, Some(iv), public_key.as_slice()).expect("Encryption failure"),
    );
    let sk_enc_b64 = hex::encode(
        encrypt(aes, key, Some(iv), secret_key.as_slice()).expect("Encryption failure"),
    );

    match locate_keys(pk_enc_b64, sk_enc_b64) {
        Ok(_) => (),
        Err(s) => panic!("{}", s),
    }
}

fn locate_keys(pk: String, sk: String) -> Result<(), String> {
    let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
    let new_dir = PathBuf::from(format!(
        "{}/{}",
        dir.data_local_dir()
            .to_str()
            .ok_or("Error getting data local dir")?,
        "PassVault"
    ));
    let sk_path = new_dir.join("secret_key.pem");
    let pk_path = new_dir.join("public_key.pem");

    println!("{:?}", new_dir);

    if !pk_path.exists() {
        fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
        // First time creation
        let mut sk_file = File::create(&sk_path).map_err(|err| format!("Error creating sk file: {}", err))?;
        let mut pk_file = File::create(&pk_path).map_err(|err| format!("Error creating pk file: {}", err))?;

        sk_file.write_all(sk.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
        pk_file.write_all(pk.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;

        println!("key created at {}", new_dir.to_str().unwrap());
        
    }

    Ok(())
}

pub fn get_keys(symmetric_key: &String) -> Result<(String, String), String> {
    let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
    let keys_dir = PathBuf::from(format!(
        "{}/{}",
        dir.data_local_dir()
            .to_str()
            .ok_or("Error getting data local dir")?,
        "PassVault"
    ));
    let sk_path = keys_dir.join("secret_key.pem");
    let pk_path = keys_dir.join("public_key.pem");

    if keys_dir.exists() {
        let sk_file = File::open(&sk_path).map_err(|err| format!("Error opening file: {}", err))?;
        let pk_file = File::open(&pk_path).map_err(|err| format!("Error opening file: {}", err))?;

        let mut sk_reader = BufReader::new(sk_file);
        let mut pk_reader = BufReader::new(pk_file);
        
        let mut pk_enc_b64: String = String::new();
        pk_reader.read_to_string(&mut pk_enc_b64).expect("Unable to read the key");

        let mut sk_enc_b64: String = String::new();
        sk_reader.read_to_string(&mut sk_enc_b64).expect("Unable to read the key");

        let sk_enc = hex::decode(sk_enc_b64).expect("Unable to decode the key");
        let pk_enc = hex::decode(pk_enc_b64).expect("Unable to decode the key");


        let my_key_32 = format!("{:0>32}", symmetric_key);

        let key = my_key_32.as_bytes();
        let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
        
        let aes = Cipher::aes_256_cbc();
        let sk = String::from_utf8(decrypt(aes, key, Some(iv), &sk_enc).expect("Unable to decrypt the key"));
        let pk = String::from_utf8(decrypt(aes, key, Some(iv), &pk_enc).expect("Unable to decrypt the key"));

        return Ok((pk.unwrap(), sk.unwrap()));

    }

    Err("Unable to find key".to_owned())
}


pub fn encrypt_data(data: &String, symmetric_key: &String) -> Result<String, String> {
    let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
    let pk_dir = PathBuf::from(format!(
        "{}/{}",
        dir.data_local_dir()
            .to_str()
            .ok_or("Error getting data local dir")?,
        "PassVault"
    ));
    
    let pk_path = pk_dir.join("public_key.pem");
    if pk_dir.exists() {
        let pk_file = File::open(&pk_path).map_err(|err| format!("Error opening file: {}", err))?;

        let mut pk_reader = BufReader::new(pk_file);

        let mut pk_enc_b64: String = String::new();
        pk_reader.read_to_string(&mut pk_enc_b64).expect("Unable to read the key");

        let pk_enc = hex::decode(pk_enc_b64).expect("Unable to decode the key");
        let my_key_32 = format!("{:0>32}", symmetric_key);

        let key = my_key_32.as_bytes();
        let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

        let aes = Cipher::aes_256_cbc();
        let pk = String::from_utf8(decrypt(aes, key, Some(iv), &pk_enc).expect("Unable to decrypt the key")).unwrap();
        
        let public_key = Rsa::public_key_from_pem(pk.as_bytes()).unwrap();
        let mut buf = vec![0; public_key.size() as usize];

        let _enc_size = public_key.public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    

        let data_enc_hex = hex::encode(buf);
        
        return Ok(data_enc_hex);
    }

    Err(String::from("Unable to find the key"))
}


pub fn decrypt_data(symmetric_key: &String) -> Result<String, String> {
    let dir = directories::BaseDirs::new().ok_or("Error getting base directories").unwrap();
    let sk_dir = PathBuf::from(format!(
        "{}/{}",
        dir.data_local_dir()
            .to_str()
            .ok_or("Error getting data local dir").unwrap(),
        "PassVault"
    ));

    let data_dir = PathBuf::from(format!(
        "{}/{}",
        dir.data_local_dir()
            .to_str()
            .ok_or("Error getting data local dir").unwrap(),
        "PassVault"
    ));
    
    let sk_path = sk_dir.join("secret_key.pem");
    let data_path = data_dir.join("accounts.config");
    if sk_dir.exists() {
        let sk_file = File::open(&sk_path).map_err(|err| format!("Error opening file: {}", err))?;
        let data_file = File::open(&data_path).map_err(|err| format!("Error opening file: {}", err))?;

        let mut sk_reader = BufReader::new(sk_file);
        let mut data_reader = BufReader::new(data_file);

        let mut sk_enc_b64: String = String::new();
        sk_reader.read_to_string(&mut sk_enc_b64).expect("Unable to read the key");

        let mut data_b64: String = String::new();
        data_reader.read_to_string(&mut data_b64).expect("Unable to read the key");

        let sk_enc = hex::decode(sk_enc_b64).expect("Unable to decode the key");
        let my_key_32 = format!("{:0>32}", symmetric_key);

        let key = my_key_32.as_bytes();
        let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

        let aes = Cipher::aes_256_cbc();
        println!{"{}", symmetric_key};
        let sk = String::from_utf8(decrypt(aes, key, Some(iv), &sk_enc).expect("Unable to decrypt the key")).unwrap();
        
        let secret_key = Rsa::private_key_from_pem(sk.as_bytes()).unwrap();
        let mut buf = vec![0; secret_key.size() as usize];

        let data = hex::decode(data_b64).unwrap();

        let _enc_size = secret_key.private_decrypt(&data, &mut buf, Padding::PKCS1).unwrap();

        return Ok(String::from_utf8(buf).unwrap());

    }
    // Add a default return value if the condition is not met
    Err("Unable to find secret key".to_owned())

}

pub fn is_pk_key_created() -> bool {
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
    let pk_path = keys_dir.join("public_key.pem");
    pk_path.exists()
}