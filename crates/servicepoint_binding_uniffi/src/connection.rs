use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct Connection {
    actual: servicepoint::Connection
}

#[uniffi::export]
impl Connection {
    #[uniffi::constructor]
    pub fn new(host: String) -> Arc<Self> {
        // TODO return Result
        let result = servicepoint::Connection::open(host).unwrap();
        Arc::new(Connection{actual: result})
    }
}
