//! An example on how to modify the image on screen without knowing the current content.

use clap::Parser;
use servicepoint::{
    Bitmap, BitmapCommand, CompressionCode, Grid, Origin, Packet, UdpSocketExt,
    FRAME_PACING, PIXEL_HEIGHT, PIXEL_WIDTH,
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

    let mut command = BitmapCommand {
        compression: CompressionCode::Uncompressed,
        bitmap: Bitmap::max_sized(),
        origin: Origin::ZERO,
    };

    command.bitmap.fill(true);

    let mut buf = [0u8; 10000];
    for x_offset in 0..PIXEL_WIDTH {
        for y in 0..PIXEL_HEIGHT {
            command.bitmap.set((x_offset + y) % PIXEL_WIDTH, y, false);
        }

        let packet: Packet = Packet::try_from(&command)
            .expect("could not turn command into packet");
        let size = packet
            .serialize_to(&mut buf)
            .expect("failed to serialize packet");
        connection
            .send(&buf[..size])
            .expect("could not send command to display");

        thread::sleep(sleep_duration);
    }
}
