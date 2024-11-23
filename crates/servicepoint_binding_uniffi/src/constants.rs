#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, uniffi::Record)]
pub struct Constants {
    pub tile_size: u64,
    pub tile_width: u64,
    pub tile_height: u64,
    pub pixel_width: u64,
    pub pixel_height: u64,
    pub pixel_count: u64,
}

#[uniffi::export]
fn get_constants() -> Constants {
Constants {
        tile_size: servicepoint::TILE_SIZE as u64,
        tile_width: servicepoint::TILE_WIDTH as u64,
        tile_height: servicepoint::TILE_HEIGHT as u64,
        pixel_width: servicepoint::PIXEL_WIDTH as u64,
        pixel_height: servicepoint::PIXEL_HEIGHT as u64,
        pixel_count: servicepoint::PIXEL_COUNT as u64,
    }
}
