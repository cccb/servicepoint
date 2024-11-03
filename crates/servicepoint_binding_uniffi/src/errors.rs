
#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum ServicePointError {
    #[error("An IO error occurred: {error}")]
    IOError {error: String},
    #[error("The specified brightness value {value} is out of range")]
    InvalidBrightness {value:u8},
}