use std::sync::{Arc, RwLock};

#[derive(uniffi::Object)]
pub struct BitVec {
    pub(crate) actual: RwLock<servicepoint::BitVec>,
}

impl BitVec {
    fn internal_new(actual: servicepoint::BitVec) -> Arc<Self> {
        Arc::new(Self {
            actual: RwLock::new(actual),
        })
    }
}

#[uniffi::export]
impl BitVec {
    #[uniffi::constructor]
    pub fn new(size: u64) -> Arc<Self> {
        Self::internal_new(servicepoint::BitVec::repeat(false, size as usize))
    }
    #[uniffi::constructor]
    pub fn load(data: Vec<u8>) -> Arc<Self> {
        Self::internal_new(servicepoint::BitVec::from_slice(&data))
    }

    #[uniffi::constructor]
    pub fn clone(other: &Arc<Self>) -> Arc<Self> {
        Self::internal_new(other.actual.read().unwrap().clone())
    }

    pub fn set(&self, index: u64, value: bool) {
        self.actual.write().unwrap().set(index as usize, value)
    }

    pub fn get(&self, index: u64) -> bool {
        self.actual
            .read()
            .unwrap()
            .get(index as usize)
            .is_some_and(move |bit| *bit)
    }

    pub fn fill(&self, value: bool) {
        self.actual.write().unwrap().fill(value)
    }

    pub fn len(&self) -> u64 {
        self.actual.read().unwrap().len() as u64
    }

    pub fn equals(&self, other: &BitVec) -> bool {
        let a = self.actual.read().unwrap();
        let b = other.actual.read().unwrap();
        *a == *b
    }
}
