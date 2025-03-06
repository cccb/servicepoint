use std::time::Duration;

/// size of a single tile in one dimension
pub const TILE_SIZE: usize = 8;

/// Display tile count in the x-direction
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{Cp437Grid, TILE_HEIGHT, TILE_WIDTH};
/// let grid = Cp437Grid::new(TILE_WIDTH, TILE_HEIGHT);
/// ```
pub const TILE_WIDTH: usize = 56;

/// Display tile count in the y-direction
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{Cp437Grid, TILE_HEIGHT, TILE_WIDTH};
/// let grid = Cp437Grid::new(TILE_WIDTH, TILE_HEIGHT);
/// ```
pub const TILE_HEIGHT: usize = 20;

/// Display width in pixels
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{PIXEL_HEIGHT, PIXEL_WIDTH, Bitmap};
/// let grid = Bitmap::new(PIXEL_WIDTH, PIXEL_HEIGHT);
/// ```
pub const PIXEL_WIDTH: usize = TILE_WIDTH * TILE_SIZE;

/// Display height in pixels
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{PIXEL_HEIGHT, PIXEL_WIDTH, Bitmap};
/// let grid = Bitmap::new(PIXEL_WIDTH, PIXEL_HEIGHT);
/// ```
pub const PIXEL_HEIGHT: usize = TILE_HEIGHT * TILE_SIZE;

/// pixel count on whole screen
pub const PIXEL_COUNT: usize = PIXEL_WIDTH * PIXEL_HEIGHT;

/// Actual hardware limit is around 28-29ms/frame. Rounded up for less dropped packets.
///
/// # Examples
///
/// ```rust
/// # use std::time::Instant;
/// # use servicepoint::{Command, CompressionCode, FRAME_PACING, Origin, Bitmap, Connection};
/// # let connection = servicepoint::connection::Fake;
/// # let pixels = Bitmap::max_sized();
/// loop {
///    let start = Instant::now();
///
///    // Change pixels here
///
///    connection.send(Command::BitmapLinearWin(
///            Origin::new(0,0),
///            pixels,
///            CompressionCode::default()
///        ))
///        .expect("send failed");
///
///    // warning: will crash if resulting duration is negative, e.g. when resuming from standby
///    std::thread::sleep(FRAME_PACING - start.elapsed());
///    # break; // prevent doctest from hanging
/// }
/// ```
pub const FRAME_PACING: Duration = Duration::from_millis(30);
