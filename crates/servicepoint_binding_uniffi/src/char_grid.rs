use crate::cp437_grid::Cp437Grid;
use servicepoint::{Grid, primitive_grid::SeriesError};
use std::convert::Into;
use std::sync::{Arc, RwLock};

#[derive(uniffi::Object)]
pub struct CharGrid {
    pub(crate) actual: RwLock<servicepoint::CharGrid>,
}

#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum CharGridError {
    #[error("Exactly one character was expected, but {value:?} was provided")]
    StringNotOneChar { value: String },
    #[error("The provided series was expected to have a length of {expected}, but was {actual}")]
    InvalidSeriesLength { actual: u64, expected: u64 },
    #[error("The index {index} was out of bounds for size {size}")]
    OutOfBounds { index: u64, size: u64 },
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

    pub fn set(
        &self,
        x: u64,
        y: u64,
        value: String,
    ) -> Result<(), CharGridError> {
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

    pub fn set_row(&self, y: u64, row: String) -> Result<(), CharGridError> {
        self.actual
            .write()
            .unwrap()
            .set_row(y as usize, &row.chars().collect::<Vec<_>>())
            .map_err(CharGridError::from)
    }

    pub fn set_col(&self, x: u64, col: String) -> Result<(), CharGridError> {
        self.actual
            .write()
            .unwrap()
            .set_row(x as usize, &col.chars().collect::<Vec<_>>())
            .map_err(CharGridError::from)
    }

    pub fn get_row(&self, y: u64) -> Result<String, CharGridError> {
        self.actual
            .read()
            .unwrap()
            .get_row(y as usize)
            .map(String::from_iter)
            .ok_or(CharGridError::OutOfBounds {index: y, size: self.height()})
    }

    pub fn get_col(&self, x: u64) -> Result<String, CharGridError> {
        self.actual
            .read()
            .unwrap()
            .get_col(x as usize)
            .map(String::from_iter)
            .ok_or(CharGridError::OutOfBounds {index: x, size: self.width()})
    }

    pub fn to_cp437(&self) -> Arc<Cp437Grid> {
        Cp437Grid::internal_new(servicepoint::Cp437Grid::from(&*self.actual.read().unwrap()))
    }
}

impl CharGrid {
    pub(crate) fn internal_new(actual: servicepoint::CharGrid) -> Arc<Self> {
        Arc::new(Self {
            actual: RwLock::new(actual),
        })
    }

    fn str_to_char(value: String) -> Result<char, CharGridError> {
        if value.len() != 1 {
            return Err(CharGridError::StringNotOneChar {
                value,
            });
        }

        let value = value.chars().nth(0).unwrap();
        Ok(value)
    }
}

impl From<SeriesError> for CharGridError {
    fn from(e: SeriesError) -> Self {
        match e {
            SeriesError::OutOfBounds { index, size } => {
                CharGridError::OutOfBounds {
                    index: index as u64,
                    size: size as u64,
                }
            }
            SeriesError::InvalidLength { actual, expected } => {
                CharGridError::InvalidSeriesLength {
                    actual: actual as u64,
                    expected: expected as u64,
                }
            }
        }
    }
}
