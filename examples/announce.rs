//! An example for how to send text to the display.

use clap::Parser;
use servicepoint::{
    CharGrid, CharGridCommand, ClearCommand, SendCommandExt, TILE_WIDTH,
};
use std::net::UdpSocket;

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

    let connection = UdpSocket::bind(&cli.destination)
        .expect("could not connect to display");

    if cli.clear {
        connection
            .send_command(ClearCommand)
            .expect("sending clear failed");
    }

    let text = cli.text.join("\n");
    let command: CharGridCommand = CharGrid::wrap_str(TILE_WIDTH, &text).into();
    connection
        .send_command(command)
        .expect("sending text failed");
}
