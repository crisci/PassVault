pub mod step {
    #[derive(Default, Debug, Clone, Copy)]
    pub struct Step {
        current: Steps,
    }

    #[derive(Debug)]
    pub enum Steps {
        Welcome,
        Login,
        SecretKeyLocation,
    }

    impl Default for Steps {
        fn default() -> Self {
            Self::Welcome
        }
    }

    impl Clone for Steps {
        fn clone(&self) -> Self {
            match self {
                Self::Welcome => Self::Welcome,
                Self::Login => Self::Login,
                Self::SecretKeyLocation => Self::SecretKeyLocation,
            }
        }
    }

    impl Copy for Steps {}

    impl Step {
        pub fn get(self) -> Steps {
            self.current
        }

        pub fn set(mut self, step: Steps) {
            self.current = step
        }

        pub fn has_next(&self) -> bool {
            match self.current {
                Steps::Welcome => true,
                Steps::Login => true,
                Steps::SecretKeyLocation => false,
            }
        }
    }
}
