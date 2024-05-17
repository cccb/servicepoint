use std::thread;
use std::time::Duration;

use clap::Parser;
use rand::{distributions, Rng};

use servicepoint2::{Command, CompressionCode, Connection, Origin, PixelGrid};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long, default_value_t = 0.5f64)]
    probability: f64,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let connection = Connection::open(&cli.destination).unwrap();
    let mut field = make_random_field(cli.probability);

    loop {
        connection
            .send(
                Command::BitmapLinearWin(
                    Origin(0, 0),
                    field.clone(),
                    CompressionCode::Bzip2,
                )
                .into(),
            )
            .expect("could not send");
        thread::sleep(Duration::from_millis(30));
        field = iteration(field);
    }
}

fn iteration(field: PixelGrid) -> PixelGrid {
    let mut next = field.clone();
    for x in 0..field.width() {
        for y in 0..field.height() {
            let old_state = field.get(x, y);
            let neighbors = count_neighbors(&field, x as i32, y as i32);

            let new_state = matches!(
                (old_state, neighbors),
                (true, 2) | (true, 3) | (false, 3)
            );
            next.set(x, y, new_state);
        }
    }
    next
}

fn count_neighbors(field: &PixelGrid, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for nx in x - 1..=x + 1 {
        for ny in y - 1..=y + 1 {
            if nx == x && ny == y {
                continue; // the cell itself does not count
            }

            if nx < 0
                || ny < 0
                || nx >= field.width() as i32
                || ny >= field.height() as i32
            {
                continue; // pixels outside the grid do not count
            }

            if !field.get(nx as usize, ny as usize) {
                continue; // dead cells do not count
            }

            count += 1;
        }
    }

    count
}

fn make_random_field(probability: f64) -> PixelGrid {
    let mut field = PixelGrid::max_sized();
    let mut rng = rand::thread_rng();
    let d = distributions::Bernoulli::new(probability).unwrap();
    for x in 0..field.width() {
        for y in 0..field.height() {
            field.set(x, y, rng.sample(d));
        }
    }
    field
}
