//! Show a brightness level test pattern on screen

use clap::Parser;

use servicepoint::Command::BitmapLinearWin;
use servicepoint::*;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
}

fn main() {
    let cli = Cli::parse();
    let connection = Connection::open(cli.destination)
        .expect("could not connect to display");

    let mut pixels = PixelGrid::max_sized();
    pixels.fill(true);

    connection
        .send(BitmapLinearWin(
            Origin::new(0, 0),
            pixels,
            CompressionCode::Uncompressed,
        ))
        .expect("send failed");

    let mut brightnesses = ByteGrid::new(TILE_WIDTH, TILE_HEIGHT);
    for (index, byte) in brightnesses.data_ref_mut().iter_mut().enumerate() {
        *byte = (index % u8::MAX as usize) as u8;
    }

    connection
        .send(Command::CharBrightness(Origin::new(0, 0), brightnesses))
        .expect("send failed");
}
