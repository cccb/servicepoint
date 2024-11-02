use std::sync::Arc;

#[uniffi::export]
trait Command: Send + Sync {}

#[derive(uniffi::Object)]
pub struct Clear {
    actual: servicepoint::Command,
}
#[uniffi::export]
impl Command for Clear {}

#[uniffi::export]
impl Clear {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        let actual = servicepoint::Command::Clear;
        Arc::new(Clear { actual })
    }
}
