use anyhow::Result;
use airportdisplay::{
    Command,
    Display,
    text::Text,
    text::Buffer,
};

fn main() -> Result<()> {
    println!("Sending hello display...");
    let display = Display::open("172.23.42.29:2342".into())?;

    let text = Command::Text(Text::Buffer(
        Buffer::from(
            "♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥\n♥ mauuu          ♥\n♥ mau mauuuunz!  ♥\n♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥".to_string())));

    display.send(Command::Clear)?;
    display.send(text)?;

    Ok(())
}
