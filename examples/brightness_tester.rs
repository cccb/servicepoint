//! Show a brightness level test pattern on screen

use clap::Parser;
use servicepoint::{
    Bitmap, BitmapCommand, Brightness, BrightnessGrid, BrightnessGridCommand,
    DataRef, Grid, SendCommandExt, TILE_HEIGHT, TILE_WIDTH,
};
use std::net::UdpSocket;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
}

fn main() {
    let cli = Cli::parse();
    let connection =
        UdpSocket::bind(cli.destination).expect("could not connect to display");

    let mut bitmap = Bitmap::max_sized();
    bitmap.fill(true);

    connection
        .send_command(BitmapCommand::from(bitmap))
        .expect("send failed");

    let max_brightness: u8 = Brightness::MAX.into();
    let mut brightnesses = BrightnessGrid::new(TILE_WIDTH, TILE_HEIGHT);
    for (index, byte) in brightnesses.data_ref_mut().iter_mut().enumerate() {
        let level = index as u8 % max_brightness;
        *byte = Brightness::try_from(level).unwrap();
    }

    let command: BrightnessGridCommand = brightnesses.into();
    connection.send_command(command).expect("send failed");
}
