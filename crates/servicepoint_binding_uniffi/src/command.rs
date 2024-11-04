use std::sync::Arc;
use servicepoint::{CompressionCode, Origin};
use crate::bitmap::Bitmap;
use crate::errors::ServicePointError;

#[derive(uniffi::Object)]
pub struct Command {
    pub(crate)actual: servicepoint::Command
}

impl Command {
    fn internal_new(actual: servicepoint::Command)-> Arc<Command>  {
        Arc::new(Command { actual })
    }
}

#[uniffi::export]
impl Command {
    #[uniffi::constructor]
    pub fn clear() -> Arc<Self> {
        Self::internal_new(servicepoint::Command::Clear)
    }

    #[uniffi::constructor]
    pub fn brightness(brightness: u8) -> Result<Arc<Self>, ServicePointError> {
        servicepoint::Brightness::try_from(brightness)
            .map_err(move |value| ServicePointError::InvalidBrightness{value})
            .map(servicepoint::Command::Brightness)
            .map(Self::internal_new)
    }

    #[uniffi::constructor]
    pub fn fade_out() -> Arc<Self> {
        Self::internal_new(servicepoint::Command::FadeOut)
    }

    #[uniffi::constructor]
    pub fn hard_reset() -> Arc<Self> {
        Self::internal_new(servicepoint::Command::HardReset)
    }
    
    #[uniffi::constructor]
    pub fn bitmap_linear_win(offset_x: u64, offset_y: u64, bitmap: &Arc<Bitmap>) -> Arc<Self> {
        let origin = Origin::new(offset_x as usize, offset_y as usize);
        let bitmap = bitmap.actual.read().unwrap().clone();
        // TODO: compression codes
        let actual = servicepoint::Command::BitmapLinearWin(origin, bitmap, CompressionCode::Uncompressed);
        Self::internal_new(actual)
    }
}
