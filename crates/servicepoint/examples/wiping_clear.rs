//! An example on how to modify the image on screen without knowing the current content.
use std::thread;
use std::time::Duration;

use clap::Parser;

use servicepoint::{bitvec::prelude::BitVec, *};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long = "duration-ms", default_value_t = 5000)]
    time: u64,
}

fn main() {
    let cli = Cli::parse();

    let sleep_duration = Duration::max(
        FRAME_PACING,
        Duration::from_millis(cli.time / PIXEL_WIDTH as u64),
    );

    let connection = Connection::open(cli.destination)
        .expect("could not connect to display");

    let mut enabled_pixels = Bitmap::new(PIXEL_WIDTH, PIXEL_HEIGHT);
    enabled_pixels.fill(true);

    for x_offset in 0..PIXEL_WIDTH {
        for y in 0..PIXEL_HEIGHT {
            enabled_pixels.set(x_offset % PIXEL_WIDTH, y, false);
        }

        // this works because the pixel grid has max size
        let pixel_data: Vec<u8> = enabled_pixels.clone().into();
        let bit_vec = BitVec::from_vec(pixel_data);

        connection
            .send(Command::BitmapLinearAnd(0, bit_vec, CompressionCode::Lzma))
            .expect("could not send command to display");
        thread::sleep(sleep_duration);
    }
}
