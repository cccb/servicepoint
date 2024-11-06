use servicepoint::{Brightness, Grid};
use std::sync::{Arc, RwLock};

#[derive(uniffi::Object)]
pub struct BrightnessGrid {
    pub(crate) actual: RwLock<servicepoint::BrightnessGrid>,
}

impl BrightnessGrid {
    fn internal_new(actual: servicepoint::BrightnessGrid) -> Arc<Self> {
        Arc::new(Self {
            actual: RwLock::new(actual),
        })
    }
}

#[uniffi::export]
impl BrightnessGrid {
    #[uniffi::constructor]
    pub fn new(width: u64, height: u64) -> Arc<Self> {
        Self::internal_new(servicepoint::BrightnessGrid::new(
            width as usize,
            height as usize,
        ))
    }

    #[uniffi::constructor]
    pub fn load(width: u64, height: u64, data: Vec<u8>) -> Arc<Self> {
        Self::internal_new(servicepoint::BrightnessGrid::saturating_load(
            width as usize,
            height as usize,
            &data,
        ))
    }

    #[uniffi::constructor]
    pub fn clone(other: &Arc<Self>) -> Arc<Self> {
        Self::internal_new(other.actual.read().unwrap().clone())
    }

    pub fn set(&self, x: u64, y: u64, value: u8) {
        self.actual.write().unwrap().set(
            x as usize,
            y as usize,
            Brightness::saturating_from(value),
        )
    }

    pub fn get(&self, x: u64, y: u64) -> u8 {
        self.actual
            .read()
            .unwrap()
            .get(x as usize, y as usize)
            .into()
    }

    pub fn fill(&self, value: u8) {
        self.actual
            .write()
            .unwrap()
            .fill(Brightness::saturating_from(value))
    }
    pub fn width(&self) -> u64 {
        self.actual.read().unwrap().width() as u64
    }

    pub fn height(&self) -> u64 {
        self.actual.read().unwrap().height() as u64
    }

    pub fn equals(&self, other: &BrightnessGrid) -> bool {
        let a = self.actual.read().unwrap();
        let b = other.actual.read().unwrap();
        *a == *b
    }
}
