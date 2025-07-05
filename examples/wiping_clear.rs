//! An example on how to modify the image on screen without knowing the current content.

use clap::Parser;
use servicepoint::{
    Bitmap, BitmapCommand, UdpSocketExt, FRAME_PACING, PIXEL_HEIGHT,
    PIXEL_WIDTH,
};
use std::{net::UdpSocket, thread, time::Duration};

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

    let connection = UdpSocket::bind_connect(cli.destination)
        .expect("could not connect to display");

    let mut enabled_pixels = Bitmap::max_sized();
    enabled_pixels.fill(true);

    for x_offset in 0..PIXEL_WIDTH {
        for y in 0..PIXEL_HEIGHT {
            enabled_pixels.set(x_offset % PIXEL_WIDTH, y, false);
        }

        let command = BitmapCommand::from(enabled_pixels.clone());
        connection
            .send_command(command)
            .expect("could not send command to display");
        thread::sleep(sleep_duration);
    }
}
