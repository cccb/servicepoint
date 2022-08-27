use anyhow::Result;
use airportdisplay::{
    Command,
    Display,
    Text,
    TextBuffer,
};

fn main() -> Result<()> {
    println!("Sending hello display...");
    let display = Display::open("172.23.42.29:2342".into())?;

    let text = Command::Text(Text::Buffer(
        TextBuffer::from(
            "♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥\n♥ mauuu          ♥\n♥ mau mauuuunz!  ♥\n♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥".into())));

    display.send(Command::Clear)?;
    display.send(text)?;

    Ok(())
}
