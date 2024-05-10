use std::thread;
use std::time::Duration;
use clap::Parser;
use servicepoint2::{Connection, Origin, PIXEL_HEIGHT, PIXEL_WIDTH, PixelGrid};
use servicepoint2::Command::BitmapLinearWin;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
}

fn main() {
    let cli = Cli::parse();
    println!("starting with args: {:?}", &cli);
    let connection = Connection::open(cli.destination).unwrap();

    let origin = Origin(0, 0);
    let mut pixels = PixelGrid::max_sized();
    for x_offset in 0..usize::MAX {
        pixels.fill(false);

        for y in 0..PIXEL_HEIGHT as usize {
            pixels.set((y + x_offset) % PIXEL_WIDTH as usize, y, true);
        }
        connection.send(BitmapLinearWin(origin, pixels.clone())).unwrap();
        thread::sleep(Duration::from_millis(14));
    }
}
