use servicepoint::Grid;
use std::convert::Into;
use std::sync::{Arc, RwLock};

#[derive(uniffi::Object)]
pub struct CharGrid {
    pub(crate) actual: RwLock<servicepoint::CharGrid>,
}

impl CharGrid {
    fn internal_new(actual: servicepoint::CharGrid) -> Arc<Self> {
        Arc::new(Self {
            actual: RwLock::new(actual),
        })
    }
}

#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum CharGridError {
    #[error("Exactly one character was expected")]
    StringNotOneChar,
}

#[uniffi::export]
impl CharGrid {
    #[uniffi::constructor]
    pub fn new(width: u64, height: u64) -> Arc<Self> {
        Self::internal_new(servicepoint::CharGrid::new(
            width as usize,
            height as usize,
        ))
    }

    #[uniffi::constructor]
    pub fn load(data: String) -> Arc<Self> {
        Self::internal_new(servicepoint::CharGrid::from(&*data))
    }

    #[uniffi::constructor]
    pub fn clone(other: &Arc<Self>) -> Arc<Self> {
        Self::internal_new(other.actual.read().unwrap().clone())
    }

    pub fn set(&self, x: u64, y: u64, value: String) -> Result<(), CharGridError> {
        let value = Self::str_to_char(value)?;
        self.actual
            .write()
            .unwrap()
            .set(x as usize, y as usize, value);
        Ok(())
    }

    pub fn get(&self, x: u64, y: u64) -> String {
        self.actual
            .read()
            .unwrap()
            .get(x as usize, y as usize)
            .into()
    }

    pub fn fill(&self, value: String) -> Result<(), CharGridError> {
        let value = Self::str_to_char(value)?;
        self.actual.write().unwrap().fill(value);
        Ok(())
    }
    pub fn width(&self) -> u64 {
        self.actual.read().unwrap().width() as u64
    }

    pub fn height(&self) -> u64 {
        self.actual.read().unwrap().height() as u64
    }

    pub fn equals(&self, other: &CharGrid) -> bool {
        let a = self.actual.read().unwrap();
        let b = other.actual.read().unwrap();
        *a == *b
    }

    pub fn as_string(&self) -> String {
        let grid = self.actual.read().unwrap();
        String::from(&*grid)
    }
}

impl CharGrid {
    fn str_to_char(value: String) -> Result<char, CharGridError> {
        if value.len() != 1 {
            return Err(CharGridError::StringNotOneChar);
        }

        let value = value.chars().nth(0).unwrap();
        Ok(value)
    }
}
