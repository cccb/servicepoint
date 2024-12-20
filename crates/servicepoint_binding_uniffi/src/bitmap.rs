use servicepoint::{DataRef, Grid};
use std::sync::{Arc, RwLock};

#[derive(uniffi::Object)]
pub struct Bitmap {
    pub(crate) actual: RwLock<servicepoint::Bitmap>,
}

impl Bitmap {
    fn internal_new(actual: servicepoint::Bitmap) -> Arc<Self> {
        Arc::new(Self {
            actual: RwLock::new(actual),
        })
    }
}

#[uniffi::export]
impl Bitmap {
    #[uniffi::constructor]
    pub fn new(width: u64, height: u64) -> Arc<Self> {
        Self::internal_new(servicepoint::Bitmap::new(
            width as usize,
            height as usize,
        ))
    }

    #[uniffi::constructor]
    pub fn new_max_sized() -> Arc<Self> {
        Self::internal_new(servicepoint::Bitmap::max_sized())
    }

    #[uniffi::constructor]
    pub fn load(width: u64, height: u64, data: Vec<u8>) -> Arc<Self> {
        Self::internal_new(servicepoint::Bitmap::load(
            width as usize,
            height as usize,
            &data,
        ))
    }

    #[uniffi::constructor]
    pub fn clone(other: &Arc<Self>) -> Arc<Self> {
        Self::internal_new(other.actual.read().unwrap().clone())
    }

    pub fn set(&self, x: u64, y: u64, value: bool) {
        self.actual
            .write()
            .unwrap()
            .set(x as usize, y as usize, value)
    }

    pub fn get(&self, x: u64, y: u64) -> bool {
        self.actual.read().unwrap().get(x as usize, y as usize)
    }

    pub fn fill(&self, value: bool) {
        self.actual.write().unwrap().fill(value)
    }
    pub fn width(&self) -> u64 {
        self.actual.read().unwrap().width() as u64
    }

    pub fn height(&self) -> u64 {
        self.actual.read().unwrap().height() as u64
    }

    pub fn equals(&self, other: &Bitmap) -> bool {
        let a = self.actual.read().unwrap();
        let b = other.actual.read().unwrap();
        *a == *b
    }

    pub fn copy_raw(&self) -> Vec<u8> {
        self.actual.read().unwrap().data_ref().to_vec()
    }
}
