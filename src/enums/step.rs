pub mod step {

    #[derive(Debug)]
    pub enum Step {
        Welcome,
        StoreSecretKey,
        GetSecretKey,
        PasswordManager,
    }

    impl Default for Step {
        fn default() -> Self {
            Self::Welcome
        }
    }

    impl Clone for Step {
        fn clone(&self) -> Self {
            match self {
                Self::Welcome => Self::Welcome,
                Self::StoreSecretKey => Self::StoreSecretKey,
                Self::GetSecretKey => Self::GetSecretKey,
                Self::PasswordManager => Self::PasswordManager,
            }
        }
    }

    impl Copy for Step {}

}
