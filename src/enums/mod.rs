#[derive(Debug)]
pub enum KeyStatus {
    CREATED(Vec<u8>),
    PRESENT(Vec<u8>),
    ERROR,
}

impl KeyStatus {
    pub fn extract(self) -> Option<Vec<u8>> {
        return match self {
            Self::CREATED(bytes) => Some(bytes),
            Self::ERROR => None,
            Self::PRESENT(bytes) => Some(bytes),
        };
    }
}
