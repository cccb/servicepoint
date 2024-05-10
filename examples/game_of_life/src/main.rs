use std::thread;
use std::time::Duration;
use rand::{distributions, Rng};
use clap::Parser;
use servicepoint2::{Connection, Origin, PixelGrid};
use servicepoint2::Command::BitmapLinearWin;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long, default_value_t = 0.5f64)]
    probability: f64,
}

fn main() {
    let cli = Cli::parse();
    println!("starting with args: {:?}", &cli);

    let connection = Connection::open(&cli.destination).unwrap();

    let mut field = PixelGrid::max_sized();

    let mut rng = rand::thread_rng();
    let d = distributions::Bernoulli::new(cli.probability).unwrap();
    for x in 0..field.width {
        for y in 0..field.height {
            field.set(x, y, rng.sample(d));
        }
    }

    let origin = Origin(0, 0);
    loop {
        connection.send(BitmapLinearWin(origin, field.clone())).expect("could not send");
        thread::sleep(Duration::from_millis(14));

        let mut next = field.clone();

        for x in 0..field.width {
            for y in 0..field.height {
                let old_state = field.get(x, y);
                let neighbors = count_neighbors(&field, x as i32, y as i32);
                let new_state = match (old_state, neighbors) {
                    (true, 2) => true,
                    (true, 3) => true,
                    (false, 3) => true,
                    _ => false
                };
                next.set(x, y, new_state);
            }
        }

        field = next;
    }
}

fn count_neighbors(field: &PixelGrid, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for nx in x - 1..=x + 1 {
        for ny in y - 1..=y + 1 {
            if nx == x && ny == y {
                continue; // the cell itself does not count
            }

            if nx < 0 || ny < 0 || nx >= field.width as i32 || ny >= field.height as i32 {
                continue; // pixels outside the grid do not count
            }

            if !field.get(nx as usize, ny as usize) {
                continue; // dead cells do not count
            }

            count += 1;
        }
    }

    return count;
}
