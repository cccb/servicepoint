use airportdisplay::{text::Text, Command, Display};
use anyhow::Result;

fn main() -> Result<()> {
    println!("Sending hello display...");
    let display = Display::open("172.23.42.29:2342".into())?;

    let text: String =
        "♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥\n♥ mauuu          ♥\n♥ mau mauuuunz!  ♥\n♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥".into();

    println!("{}", text);

    display.send(Command::Clear)?;
    display.send(Command::Text(Text::Buffer(text.into())))?;

    Ok(())
}
