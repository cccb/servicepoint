use clap::Parser;

use servicepoint2::{ByteGrid, Command, Connection, Origin};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long, num_args = 1.., value_delimiter = '\n')]
    text: Vec<String>,
    #[arg(short, long, default_value_t = true)]
    clear: bool,
}

/// example: `cargo run -- --text "Hallo,
/// CCCB"`

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let cli = Cli::parse();

    let connection = Connection::open(&cli.destination).unwrap();
    if cli.clear {
        connection.send(Command::Clear).unwrap();
    }

    let mut max_width = 0;
    for l in cli.text.iter() {
        if l.len() > max_width {
            max_width = l.len()
        }
    }

    let mut chars = ByteGrid::new(max_width, max_width * cli.text.len());
    for y in 0..cli.text.len() {
        let row = &cli.text[y];
        for x in 0..max_width {
            if x >= row.len() {
                continue;
            }

            chars.set(x, y, row.chars().nth(x).unwrap().try_into().unwrap());
        }
    }

    connection
        .send(Command::Cp437Data(Origin::top_left(), chars))
        .unwrap();
}
