//! A simple example for how to set brightnesses for tiles on the screen.
//! Continuously changes the tiles in a random window to random brightnesses.

use clap::Parser;
use rand::Rng;
use servicepoint::{
    Bitmap, BitmapCommand, Brightness, BrightnessGrid, BrightnessGridCommand,
    GlobalBrightnessCommand, Grid, Origin, UdpSocketExt, TILE_HEIGHT,
    TILE_WIDTH,
};
use std::{net::UdpSocket, time::Duration};

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
    let cli = Cli::parse();

    let connection = UdpSocket::bind_connect(cli.destination)
        .expect("could not connect to display");
    let wait_duration = Duration::from_millis(cli.wait_ms);

    // put all pixels in on state
    if cli.enable_all {
        let mut filled_grid = Bitmap::max_sized();
        filled_grid.fill(true);

        let command = BitmapCommand::from(filled_grid);
        connection.send_command(command).expect("send failed");
    }

    // set all pixels to the same random brightness
    let mut rng = rand::rng();
    let command: GlobalBrightnessCommand = rng.random::<Brightness>().into();
    connection.send_command(command).unwrap();

    // continuously update random windows to new random brightness
    loop {
        let min_size = 1;
        let x = rng.random_range(0..TILE_WIDTH - min_size);
        let y = rng.random_range(0..TILE_HEIGHT - min_size);

        let w = rng.random_range(min_size..=TILE_WIDTH - x);
        let h = rng.random_range(min_size..=TILE_HEIGHT - y);

        let origin = Origin::new(x, y);
        let mut luma = BrightnessGrid::new(w, h);

        for y in 0..h {
            for x in 0..w {
                luma.set(x, y, rng.random());
            }
        }

        connection
            .send_command(BrightnessGridCommand { origin, grid: luma })
            .unwrap();
        std::thread::sleep(wait_duration);
    }
}
