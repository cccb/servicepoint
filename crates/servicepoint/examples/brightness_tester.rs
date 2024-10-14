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
    let connection = Connection::open(cli.destination)
        .expect("could not connect to display");

    let mut pixels = Bitmap::max_sized();
    pixels.fill(true);

    let command = Command::BitmapLinearWin(
        Origin::new(0, 0),
        pixels,
        CompressionCode::Uncompressed,
    );
    connection.send(command).expect("send failed");

    let max_brightness = usize::from(u8::from(Brightness::MAX));
    let mut brightnesses = BrightnessGrid::new(TILE_WIDTH, TILE_HEIGHT);
    for (index, byte) in brightnesses.data_ref_mut().iter_mut().enumerate() {
        *byte = Brightness::try_from((index % max_brightness) as u8).unwrap();
    }

    connection
        .send(Command::CharBrightness(Origin::new(0, 0), brightnesses))
        .expect("send failed");
}
