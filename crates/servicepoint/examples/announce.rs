//! An example for how to send text to the display.

use clap::Parser;

use servicepoint::{CharGrid, Command, Connection, Origin, TILE_WIDTH};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(
        short,
        long,
        default_value = "localhost:2342",
        help = "Address of the display"
    )]
    destination: String,
    #[arg(short, long, num_args = 1.., value_delimiter = '\n',
        help = "Text to send - specify multiple times for multiple lines")]
    text: Vec<String>,
    #[arg(
        short,
        long,
        default_value_t = true,
        help = "Clear screen before sending text"
    )]
    clear: bool,
}

/// example: `cargo run -- --text "Hallo" --text "CCCB"`
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

    let text = cli
        .text
        .iter()
        .flat_map(move |x| {
            x.chars()
                .collect::<Vec<_>>()
                .chunks(TILE_WIDTH)
                .map(|c| String::from_iter(c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .join("\n");

    let grid = CharGrid::from(text);
    connection
        .send(Command::Utf8Data(Origin::ZERO, grid))
        .expect("sending text failed");
}
