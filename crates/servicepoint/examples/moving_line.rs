//! A simple example for how to send pixel data to the display.

use std::thread;

use clap::Parser;

use servicepoint::*;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
}

fn main() {
    let connection = Connection::open(Cli::parse().destination)
        .expect("could not connect to display");

    let mut pixels = PixelGrid::max_sized();
    for x_offset in 0..usize::MAX {
        pixels.fill(false);

        for y in 0..PIXEL_HEIGHT {
            pixels.set((y + x_offset) % PIXEL_WIDTH, y, true);
        }

        let command = Command::BitmapLinearWin(
            Origin::new(0, 0),
            pixels.clone(),
            CompressionCode::Lzma,
        );
        connection.send(command).expect("send failed");
        thread::sleep(FRAME_PACING);
    }
}
