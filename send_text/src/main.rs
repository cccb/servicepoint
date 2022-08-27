
use anyhow::Result;

use airportdisplay::{
    Display,
    Command,
    Text,
    TextBuffer,
};

use std::io;

fn main() -> Result<()> {

    // Read buffer
    let mut buffer = String::new();
    for line in io::stdin().lines() {
        if let Ok(line) = line {
            buffer.push_str(line.as_str()); 
            buffer.push('\n');
        }
    }

    let display = Display::open("172.23.42.29:2342".into())?;
    let cmd = Command::Text(Text::Buffer(TextBuffer::from(buffer)));
    
    display.send(Command::Clear)?;
    display.send(cmd)?;

    Ok(())
}

