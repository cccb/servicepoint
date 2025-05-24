//! An example on how to use the provided infrastructure to implement custom commands.

use rand::Rng;
use servicepoint::{
    Brightness, GlobalBrightnessCommand, Header, Packet, UdpSocketExt,
};
use std::{fmt::Debug, net::UdpSocket};

/// Command that sets the brightness to zero globally.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ZeroBrightnessCommand;

impl Into<Packet> for ZeroBrightnessCommand {
    fn into(self) -> Packet {
        GlobalBrightnessCommand::from(Brightness::MIN).into()
    }
}

/// Command that turns into a random packet.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FuzzyCommand;

impl TryInto<Packet> for FuzzyCommand {
    type Error = ();

    fn try_into(self) -> Result<Packet, Self::Error> {
        let mut rng = rand::rng();
        Ok(Packet {
            payload: None,
            header: Header {
                command_code: rng.random(),
                a: rng.random(),
                b: rng.random(),
                c: rng.random(),
                d: rng.random(),
            },
        })
    }
}

fn main() {
    let connection = UdpSocket::bind_connect("172.23.42.29:2342")
        .expect("could not connect to display");

    for _ in 0..100 {
        connection.send_command(FuzzyCommand).unwrap()
    }
    connection.send_command(ZeroBrightnessCommand).unwrap();
}
