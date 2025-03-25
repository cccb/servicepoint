//! A simple example for how to send pixel data to the display.

use clap::Parser;
use servicepoint::{
    Bitmap, BitmapCommand, Connection, Grid, UdpConnection, FRAME_PACING,
    PIXEL_HEIGHT, PIXEL_WIDTH,
};
use std::thread;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
}

fn main() {
    let connection = UdpConnection::open(Cli::parse().destination)
        .expect("could not connect to display");

    let mut bitmap = Bitmap::max_sized();
    for x_offset in 0..usize::MAX {
        bitmap.fill(false);

        for y in 0..PIXEL_HEIGHT {
            bitmap.set((y + x_offset) % PIXEL_WIDTH, y, true);
        }

        let command = BitmapCommand::from(bitmap.clone());
        connection.send(command).expect("send failed");
        thread::sleep(FRAME_PACING);
    }
}
