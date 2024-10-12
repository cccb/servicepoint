//! An example for how to send text to the display.

use clap::Parser;

use servicepoint::{CharGrid, Command, Connection, Cp437Grid, Origin};

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

    let text = cli.text.iter().fold(String::new(), move |str, line| {
        let is_first = str.is_empty();
        str + if is_first { "" } else { "\n" } + line
    });

    let grid = CharGrid::from(&*text);
    let cp437_grid = Cp437Grid::from(&grid);

    connection
        .send(Command::Cp437Data(Origin::new(0, 0), cp437_grid))
        .expect("sending text failed");
}
