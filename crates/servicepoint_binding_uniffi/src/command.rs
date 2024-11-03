use std::sync::Arc;
use crate::errors::ServicePointError;

#[derive(uniffi::Object)]
pub struct Command {
    pub(crate)actual: servicepoint::Command
}

fn actual_into_arc(actual: servicepoint::Command) -> Arc<Command> {
    Arc::new(Command { actual })
}

#[uniffi::export]
impl Command {
    #[uniffi::constructor]
    pub fn clear() -> Arc<Self> {
        actual_into_arc(servicepoint::Command::Clear)
    }

    #[uniffi::constructor]
    pub fn brightness(brightness: u8) -> Result<Arc<Self>, ServicePointError> {
        servicepoint::Brightness::try_from(brightness)
            .map_err(move |value| ServicePointError::InvalidBrightness{value})
            .map(servicepoint::Command::Brightness)
            .map(actual_into_arc)
    }

    #[uniffi::constructor]
    pub fn fade_out() -> Arc<Self> {
        actual_into_arc(servicepoint::Command::FadeOut)
    }

    #[uniffi::constructor]
    pub fn hard_reset() -> Arc<Self> {
        actual_into_arc(servicepoint::Command::HardReset)
    }
}
