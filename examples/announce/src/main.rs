use clap::Parser;
use servicepoint2::{Command, Connection, Origin, Size, Window};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long, num_args = 1.., value_delimiter = '\n')]
    text: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("starting with args: {:?}", &cli);

    let connection = Connection::open(&cli.destination).unwrap();

    let mut max_width = 0;
    for l in cli.text.iter() {
        if l.len() > max_width {
            max_width = l.len()
        }
    }

    let mut data = vec!(0; max_width * cli.text.len());
    for y in 0..cli.text.len() {
        let row = &cli.text[y];
        for x in 0..max_width {
            if x >= row.len() {
                continue;
            }

            data[x + y * max_width] = row.chars().nth(x).unwrap().try_into().unwrap();
        }
    }

    let window = Window(Origin(0, 0), Size(max_width as u16, cli.text.len() as u16));
    let command = Command::Cp437Data(window, data);

    connection.send(Command::Clear).unwrap();
    connection.send(command).unwrap()
}
