use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use arboard::Clipboard;
use directories::BaseDirs;
use openssl::{
    rand,
    symm::{decrypt, encrypt, Cipher},
};
use ::rand::{thread_rng, Rng};
use rfd::FileDialog;

pub mod utils {
    use std::{fs::File, path::PathBuf};

    pub fn create_passvault_files() {
        let dir = match directories::BaseDirs::new() {
            Some(dir) => dir,
            None => return,
        };
        let dir_str = match dir.data_local_dir().to_str() {
            Some(s) => s,
            None => return,
        };

        let passvault_path = PathBuf::from(format!("{}/{}", dir_str, "PassVault"));
        let acccounts_path = passvault_path.join("accounts.dat");
        File::create(&acccounts_path).expect("Error creating accounts file");
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

pub fn generate_key_pair(aes_path: String, password: String) -> Result<Vec<u8>> {
    let pwd_32 = utils::_pad32(&password);
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let aes = Cipher::aes_256_cbc();
    let mut aes_key = [0; 32];
    rand::rand_bytes(&mut aes_key)?;

    let aes_key_enc = encrypt(aes, pwd_32.as_bytes(), Some(iv), &aes_key)?;
    let aes_key_enc_hex = hex::encode(aes_key_enc);

    locate_keys(aes_key_enc_hex, aes_path)?;
    Ok(aes_key.to_vec())
}

fn locate_keys(aes_key: String, aes_path: String) -> anyhow::Result<()> {
    let aes_dir = PathBuf::from(aes_path);
    let aes_file_path = aes_dir.join("aes.dat");

    let mut file = File::create(&aes_file_path)?;
    file.write_all(aes_key.as_bytes())?;

    Ok(())
}

pub fn check_decryption_key(aes_path: String, password: String) -> anyhow::Result<Vec<u8>> {
    let pwd_32 = utils::_pad32(&password);
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let aes_key_enc_b64 = fs::read(aes_path)?;

    let aes = Cipher::aes_256_cbc();
    let aes_key_enc = hex::decode(aes_key_enc_b64)?;
    let aes_key = decrypt(aes, pwd_32.as_bytes(), Some(iv), &aes_key_enc)?;

    Ok(aes_key)
}

pub fn encrypt_data(data: &String, symmetric_key: &[u8]) -> anyhow::Result<String> {
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let aes = Cipher::aes_256_cbc();
    let data_enc = encrypt(aes, symmetric_key, Some(iv), data.as_bytes())?;
    let data_enc_hex = hex::encode(data_enc);

    return Ok(data_enc_hex);
}

pub fn decrypt_data(data_enc_hex: &String, symmetric_key: &[u8]) -> anyhow::Result<String> {
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let aes = Cipher::aes_256_cbc();

    let data_enc = hex::decode(data_enc_hex)?;
    let data = decrypt(aes, symmetric_key, Some(iv), &data_enc)?;

    let data_str = String::from_utf8(data)?;

    Ok(data_str)
}

pub fn is_key_created() -> bool {
    let base_dir = BaseDirs::new().unwrap();
    let passvault_path = base_dir.data_local_dir().join("PassVault");
    let accounts_path = passvault_path.join("accounts.dat");
    accounts_path.exists()
}

pub fn select_path() -> Option<String> {
    let dir = directories::UserDirs::new().unwrap();
    let home_dir = dir.home_dir();
    let result = match is_key_created() {
        true => FileDialog::new().set_directory(home_dir).pick_file(),
        false => FileDialog::new().set_directory(home_dir).pick_folder(),
    };

    match result {
        Some(selected_path) => Some(selected_path.into_os_string().into_string().unwrap()),
        None => None,
    }
}

pub fn generate_password() -> String {
    let mut rng = thread_rng();
    let password: String = (0..28)
        .map(|i| {
            if i%7 == 0 && i != 0 {
                let rnd_special = rng.gen_range(0..4);
                let c = match rnd_special {
                    0 => '!',
                    1 => '@',
                    2 => '_',
                    3 => '-',
                    _ => unreachable!(),
                };
                return c;
            }
            let idx = rng.gen_range(0..62);
            let c = match idx {
                0..=25 => (b'a' + idx) as char,
                26..=51 => (b'A' + idx - 26) as char,
                52..=61 => (b'0' + idx - 52) as char,
                _ => unreachable!(),
            };
            c
        })
        .collect();
    password

}

pub async fn copy_to_clipboard(password: String) -> Result<(), String> {
    let mut ctx = Clipboard::new().map_err(|err| format!("{:?}", err)).expect("Error");
    let binding = password.clone();
    tokio::task::spawn_blocking(move ||{   
            ctx.set_text(binding).map(|_| ()).map_err(|err| format!("{:?}", err))
    }).await.expect("Blocking task to finish")
}