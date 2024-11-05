use crate::bitmap::Bitmap;
use crate::bitvec::BitVec;
use crate::brightness_grid::BrightnessGrid;
use crate::compression_code::CompressionCode;
use crate::cp437_grid::Cp437Grid;
use crate::errors::ServicePointError;
use servicepoint::Origin;
use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct Command {
    pub(crate) actual: servicepoint::Command,
}

impl Command {
    fn internal_new(actual: servicepoint::Command) -> Arc<Command> {
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
            .map_err(move |value| ServicePointError::InvalidBrightness {
                value,
            })
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
    pub fn bitmap_linear_win(
        offset_x: u64,
        offset_y: u64,
        bitmap: &Arc<Bitmap>,
        compression: CompressionCode,
    ) -> Arc<Self> {
        let origin = Origin::new(offset_x as usize, offset_y as usize);
        let bitmap = bitmap.actual.read().unwrap().clone();
        let actual = servicepoint::Command::BitmapLinearWin(
            origin,
            bitmap,
            servicepoint::CompressionCode::try_from(compression as u16)
                .unwrap(),
        );
        Self::internal_new(actual)
    }

    #[uniffi::constructor]
    pub fn char_brightness(
        offset_x: u64,
        offset_y: u64,
        grid: &Arc<BrightnessGrid>,
    ) -> Arc<Self> {
        let origin = Origin::new(offset_x as usize, offset_y as usize);
        let grid = grid.actual.read().unwrap().clone();
        let actual = servicepoint::Command::CharBrightness(origin, grid);
        Self::internal_new(actual)
    }

    #[uniffi::constructor]
    pub fn bitmap_linear(
        offset: u64,
        bitmap: &Arc<BitVec>,
        compression: CompressionCode,
    ) -> Arc<Self> {
        let bitmap = bitmap.actual.read().unwrap().clone();
        let actual = servicepoint::Command::BitmapLinear(
            offset as usize,
            bitmap,
            servicepoint::CompressionCode::try_from(compression as u16)
                .unwrap(),
        );
        Self::internal_new(actual)
    }

    #[uniffi::constructor]
    pub fn bitmap_linear_and(
        offset: u64,
        bitmap: &Arc<BitVec>,
        compression: CompressionCode,
    ) -> Arc<Self> {
        let bitmap = bitmap.actual.read().unwrap().clone();
        let actual = servicepoint::Command::BitmapLinearAnd(
            offset as usize,
            bitmap,
            servicepoint::CompressionCode::try_from(compression as u16)
                .unwrap(),
        );
        Self::internal_new(actual)
    }

    #[uniffi::constructor]
    pub fn bitmap_linear_or(
        offset: u64,
        bitmap: &Arc<BitVec>,
        compression: CompressionCode,
    ) -> Arc<Self> {
        let bitmap = bitmap.actual.read().unwrap().clone();
        let actual = servicepoint::Command::BitmapLinearOr(
            offset as usize,
            bitmap,
            servicepoint::CompressionCode::try_from(compression as u16)
                .unwrap(),
        );
        Self::internal_new(actual)
    }

    #[uniffi::constructor]
    pub fn bitmap_linear_xor(
        offset: u64,
        bitmap: &Arc<BitVec>,
        compression: CompressionCode,
    ) -> Arc<Self> {
        let bitmap = bitmap.actual.read().unwrap().clone();
        let actual = servicepoint::Command::BitmapLinearXor(
            offset as usize,
            bitmap,
            servicepoint::CompressionCode::try_from(compression as u16)
                .unwrap(),
        );
        Self::internal_new(actual)
    }

    #[uniffi::constructor]
    pub fn cp437_data(
        offset_x: u64,
        offset_y: u64,
        grid: &Arc<Cp437Grid>,
    ) -> Arc<Self> {
        let origin = Origin::new(offset_x as usize, offset_y as usize);
        let grid = grid.actual.read().unwrap().clone();
        let actual = servicepoint::Command::Cp437Data(origin, grid);
        Self::internal_new(actual)
    }

    #[uniffi::constructor]
    pub fn clone(other: &Arc<Self>) -> Arc<Self> {
        Self::internal_new(other.actual.clone())
    }
}
