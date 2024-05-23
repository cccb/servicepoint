use anyhow::Result;

use airportdisplay::{text::Buffer, Command, Display};

use std::io;

/// Send text read from stdio to the display
fn main() -> Result<()> {
    // Read stdin into buffer
    let mut text = String::new();
    for line in io::stdin().lines() {
        text.push_str(line?.as_str());
        text.push('\n');
    }

    // Send content to display
    let display = Display::connect("172.23.42.29:2342".into())?;

    display.send(Command::Clear)?;
    display.send(Buffer::from(text).into())?;

    Ok(())
}
