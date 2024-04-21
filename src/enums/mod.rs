
pub mod step;
pub mod error;

#[derive(Debug)]
pub enum Modal {
    ADD,
    EDIT
}

impl Default for Modal {
    fn default() -> Self {
        Self::ADD
    }
}