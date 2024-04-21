pub mod error {
    
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum CryptoError {
        #[error("Encryption/Decryption error")]
        Encryption(#[from] openssl::error::ErrorStack),

        #[error("Characters not utf8 representable")]
        ParsingError(#[from] std::string::FromUtf8Error),

        #[error("Error decoding hex string")]   
        FromHexError(#[from] hex::FromHexError),
        
        #[error("IO Error")]
        IOError(#[from] std::io::Error),

        #[error("unknown data store error")]
        Unknown,
    }

}