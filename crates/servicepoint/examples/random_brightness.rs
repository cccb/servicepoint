//! A simple example for how to set brightnesses for tiles on the screen.
//! Continuously changes the tiles in a random window to random brightnesses.

use std::time::Duration;

use clap::Parser;
use rand::Rng;

use servicepoint::Command::{BitmapLinearWin, Brightness, CharBrightness};
use servicepoint::*;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long, default_value_t = true)]
    enable_all: bool,
    #[arg(short, long, default_value_t = 100, allow_negative_numbers = false)]
    wait_ms: u64,
}

fn main() {
    let cli = Cli::parse();

    let connection = Connection::open(cli.destination)
        .expect("could not connect to display");
    let wait_duration = Duration::from_millis(cli.wait_ms);

    // put all pixels in on state
    if cli.enable_all {
        let mut filled_grid = Bitmap::max_sized();
        filled_grid.fill(true);

        let command = BitmapLinearWin(
            Origin::ZERO,
            filled_grid,
            CompressionCode::Lzma,
        );
        connection.send(command).expect("send failed");
    }

    // set all pixels to the same random brightness
    let mut rng = rand::thread_rng();
    connection.send(Brightness(rng.gen())).unwrap();

    // continuously update random windows to new random brightness
    loop {
        let min_size = 1;
        let x = rng.gen_range(0..TILE_WIDTH - min_size);
        let y = rng.gen_range(0..TILE_HEIGHT - min_size);

        let w = rng.gen_range(min_size..=TILE_WIDTH - x);
        let h = rng.gen_range(min_size..=TILE_HEIGHT - y);

        let origin = Origin::new(x, y);
        let mut luma = BrightnessGrid::new(w, h);

        for y in 0..h {
            for x in 0..w {
                luma.set(x, y, rng.gen());
            }
        }

        connection.send(CharBrightness(origin, luma)).unwrap();
        std::thread::sleep(wait_duration);
    }
}
