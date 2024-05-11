use std::time::Duration;
use clap::Parser;
use rand::Rng;
use servicepoint2::{ByteGrid, Connection, Origin, PixelGrid, TILE_HEIGHT, TILE_WIDTH};
use servicepoint2::Command::{Brightness, CharBrightness, BitmapLinearWin};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long, default_value_t = true)]
    enable_all: bool,
    #[arg(short, long, default_value_t = 100, allow_negative_numbers = false)]
    wait_ms: u64,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let cli = Cli::parse();

    let connection = Connection::open(cli.destination).unwrap();
    let wait_duration = Duration::from_millis(cli.wait_ms);

    // put all pixels in on state
    if cli.enable_all {
        let mut filled_grid = PixelGrid::max_sized();
        filled_grid.fill(true);
        connection.send(BitmapLinearWin(Origin::top_left(), filled_grid)).unwrap();
    }

    // set all pixels to the same random brightness
    let mut rng = rand::thread_rng();
    connection.send(Brightness(rng.gen())).unwrap();

    // continuously update random windows to new random brightness
    loop {
        let min_size = 1;
        let x: u16 = rng.gen_range(0..TILE_WIDTH - min_size);
        let y: u16 = rng.gen_range(0..TILE_HEIGHT - min_size);

        let w: u16 = rng.gen_range(min_size..=TILE_WIDTH - x);
        let h: u16 = rng.gen_range(min_size..=TILE_HEIGHT - y);

        let origin = Origin(x, y);
        let mut luma = ByteGrid::new(w as usize, h as usize);

        for y in 0..h as usize {
            for x in 0..w as usize {
                luma.set(x, y, rng.gen());
            }
        }

        connection.send(CharBrightness(origin, luma)).unwrap();
        std::thread::sleep(wait_duration);
    }
}
