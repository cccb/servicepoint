use std::thread;
use std::time::Duration;

use clap::Parser;

use servicepoint2::Command::BitmapLinearAnd;
use servicepoint2::{
    BitVec, CompressionCode, Connection, PixelGrid, PIXEL_HEIGHT, PIXEL_WIDTH,
};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long = "duration-ms", default_value_t = 5000)]
    time: u64,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let connection = Connection::open(cli.destination).unwrap();
    let sleep_duration = Duration::from_millis(cli.time / PIXEL_WIDTH as u64);

    let mut enabled_pixels =
        PixelGrid::new(PIXEL_WIDTH as usize, PIXEL_HEIGHT as usize);
    enabled_pixels.fill(true);

    for x_offset in 0..PIXEL_WIDTH as usize {
        for y in 0..PIXEL_HEIGHT as usize {
            enabled_pixels.set(x_offset % PIXEL_WIDTH as usize, y, false);
        }

        // this works because the pixel grid has max size
        let pixel_data: Vec<u8> = enabled_pixels.clone().into();
        let bit_vec = BitVec::from(&*pixel_data);

        connection
            .send(BitmapLinearAnd(0, bit_vec, CompressionCode::Gz))
            .unwrap();
        thread::sleep(sleep_duration);
    }
}
