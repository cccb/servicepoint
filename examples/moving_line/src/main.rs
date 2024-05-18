use std::thread;

use clap::Parser;

use servicepoint2::{Command, CompressionCode, Connection, Origin, PixelGrid, FRAME_PACING, PIXEL_HEIGHT, PIXEL_WIDTH, Grid};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
}

fn main() {
    env_logger::init();

    let connection = Connection::open(Cli::parse().destination).unwrap();

    let mut pixels = PixelGrid::max_sized();
    for x_offset in 0..usize::MAX {
        pixels.fill(false);

        for y in 0..PIXEL_HEIGHT as usize {
            pixels.set((y + x_offset) % PIXEL_WIDTH as usize, y, true);
        }
        connection
            .send(
                Command::BitmapLinearWin(
                    Origin(0, 0),
                    pixels.clone(),
                    CompressionCode::Lzma,
                )
                .into(),
            )
            .unwrap();
        thread::sleep(FRAME_PACING);
    }
}
