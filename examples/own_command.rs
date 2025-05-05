use servicepoint::{Brightness, GlobalBrightnessCommand, Header, Packet, UdpSocketExt};
use std::{fmt::Debug, net::UdpSocket};
use rand::Rng;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ZeroBrightnessCommand;

impl TryInto<Packet> for ZeroBrightnessCommand {
    type Error = ();

    fn try_into(self) -> Result<Packet, Self::Error> {
        GlobalBrightnessCommand::from(Brightness::MIN).into()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FuzzyCommand;

impl TryInto<Packet> for FuzzyCommand {
    type Error = ();

    fn try_into(self) -> Result<Packet, Self::Error> {
        let mut rng = rand::thread_rng();
        Ok(Packet {
            payload: None,
            header: Header {
                command_code: rng.gen(),
                a: rng.gen(),
                b: rng.gen(),
                c: rng.gen(),
                d: rng.gen(),
            }
        })
    }
}

fn main() {
    let connection = UdpSocket::bind_connect("localhost:2342")
        .expect("could not connect to display");

    for _ in 0..100 {
        connection.send_command(FuzzyCommand).unwrap()
    }
    connection.send_command(ZeroBrightnessCommand).unwrap();
}
