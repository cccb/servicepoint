use std::{sync::Arc};

#[derive(uniffi::Object)]
pub struct Connection {
    actual: servicepoint::Connection,
}

#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum ConnectionError {
    #[error("An IO error occured: {error}")]
    IOError {
        error: String}
}

#[uniffi::export]
impl Connection {
    #[uniffi::constructor]
    pub fn new(host: String) -> Result<Arc<Self>, ConnectionError> {
        servicepoint::Connection::open(host)
            .map(|actual|Arc::new(Connection { actual}) )
            .map_err(|err| ConnectionError::IOError { error:  err.to_string()})
    }
}
