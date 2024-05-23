use airportdisplay::{text::Buffer, Command, Display};
use anyhow::Result;

fn main() -> Result<()> {
    println!("Sending hello display...");
    let display = Display::connect("172.23.42.29:2342".into())?;

    let text: String =
        "♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥\n♥ mauuu          ♥\n♥ mau mauuuunz!  ♥\n♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥".into();

    println!("{}", text);

    display.send(Command::Clear)?;
    display.send(Buffer::from(text).into())?;


    Ok(())
}
