//! Show a brightness level test pattern on screen

use clap::Parser;
use servicepoint::*;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
}

fn main() {
    let cli = Cli::parse();
    let connection = UdpConnection::open(cli.destination)
        .expect("could not connect to display");

    let mut bitmap = Bitmap::max_sized();
    bitmap.fill(true);

    let command = BitmapCommand {
        bitmap,
        origin: Origin::ZERO,
        compression: CompressionCode::default(),
    };
    connection.send(command).expect("send failed");

    let max_brightness: u8 = Brightness::MAX.into();
    let mut brightnesses = BrightnessGrid::new(TILE_WIDTH, TILE_HEIGHT);
    for (index, byte) in brightnesses.data_ref_mut().iter_mut().enumerate() {
        let level = index as u8 % max_brightness;
        *byte = Brightness::try_from(level).unwrap();
    }

    let command = BrightnessGridCommand {
        origin: Origin::ZERO,
        grid: brightnesses,
    };
    connection.send(command).expect("send failed");
}
