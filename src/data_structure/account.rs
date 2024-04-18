pub mod account {
    
    #[derive(Debug, Clone, Default)]
    pub struct Account {
        host: String, 
        username: String,
        key: String, 
    }

    impl Account {
        pub fn new(host: String, username: String, key: String) -> Self {
            Self { host, username, key }
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
            
    }

    pub fn decrypt_key(sk: String) -> String {
        String::from("Decrypted")
    }
}