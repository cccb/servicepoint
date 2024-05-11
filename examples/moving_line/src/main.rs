use std::thread;
use std::time::Duration;

use clap::Parser;

use servicepoint2::Command::BitmapLinearWin;
use servicepoint2::{Connection, Origin, PixelGrid, PIXEL_HEIGHT, PIXEL_WIDTH};

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

    let mut pixels = PixelGrid::max_sized();
    for x_offset in 0..usize::MAX {
        pixels.fill(false);

        for y in 0..PIXEL_HEIGHT as usize {
            pixels.set((y + x_offset) % PIXEL_WIDTH as usize, y, true);
        }
        connection
            .send(BitmapLinearWin(Origin::top_left(), pixels.clone()))
            .unwrap();
        thread::sleep(Duration::from_millis(14));
    }
}
