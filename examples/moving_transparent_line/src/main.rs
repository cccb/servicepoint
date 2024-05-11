use std::thread;
use std::time::Duration;

use clap::Parser;

use servicepoint2::{BitVec, Connection, PIXEL_HEIGHT, PIXEL_WIDTH, PixelGrid};
use servicepoint2::Command::{BitmapLinearOr, BitmapLinearXor};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let connection = Connection::open(Cli::parse().destination).unwrap();

    loop {
        let mut last = BitVec::new(PIXEL_WIDTH as usize * PIXEL_HEIGHT as usize);
        for x_offset in 0..PIXEL_WIDTH as usize {
            let mut pixels = PixelGrid::max_sized();
            pixels.fill(false);

            for y in 0..PIXEL_HEIGHT as usize {
                pixels.set((y + x_offset) % PIXEL_WIDTH as usize, y, true);
            }

            // this works because the pixel grid has max size
            let pixel_data: Vec<u8> = pixels.into();
            let bit_vec = BitVec::load(&*pixel_data);

            // remove pixels from last iteration
            connection.send(BitmapLinearXor(0, last)).unwrap();
            // reduces dropped packages
            thread::sleep(Duration::from_millis(1));
            // add pixels from this iteration
            connection.send(BitmapLinearOr(0, bit_vec.clone())).unwrap();

            last = bit_vec;
            thread::sleep(Duration::from_millis(1000));
        }
    }
}
