use std::sync::Arc;

use crate::command::Command;
use crate::errors::ServicePointError;

#[derive(uniffi::Object)]
pub struct Connection {
    actual: servicepoint::Connection,
}

#[uniffi::export]
impl Connection {
    #[uniffi::constructor]
    pub fn new(host: String) -> Result<Arc<Self>, ServicePointError> {
        servicepoint::Connection::open(host)
            .map(|actual|Arc::new(Connection { actual}) )
            .map_err(|err| ServicePointError::IOError { error:  err.to_string()})
    }

    pub fn send(&self, command: Arc<Command>) -> Result<(), ServicePointError> {
        self.actual.send(command.actual.clone())
            .map_err(|err| ServicePointError::IOError { error: format!("{err:?}")})
    }
}
