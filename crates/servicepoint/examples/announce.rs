//! An example for how to send text to the display.

use clap::Parser;

use servicepoint::{ByteGrid, Command, Connection, Grid, Origin};

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
    let mut cli = Cli::parse();
    if cli.text.is_empty() {
        cli.text.push("Hello, CCCB!".to_string());
    }

    let connection = Connection::open(&cli.destination)
        .expect("could not connect to display");

    if cli.clear {
        connection
            .send(Command::Clear)
            .expect("sending clear failed");
    }

    let max_width = cli.text.iter().map(|t| t.len()).max().unwrap();

    let mut chars = ByteGrid::new(max_width, cli.text.len());
    for y in 0..cli.text.len() {
        let row = &cli.text[y];

        for (x, char) in row.chars().enumerate() {
            let char = char.try_into().expect("invalid input char");
            chars.set(x, y, char);
        }
    }

    connection
        .send(Command::Cp437Data(Origin::new(0, 0), chars))
        .expect("sending text failed");
}
