pub mod step {

    #[derive(Debug)]
    pub enum Step {
        Welcome,
        StoreSecretKey,
        GetSecretKey,
        PasswordManager,
        PasswordCreation,
        PasswordCheck
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
                Self::PasswordCreation => Self::PasswordCreation,
                Self::PasswordCheck => Self::PasswordCheck
            }
        }
    }

    impl Copy for Step {}

}
