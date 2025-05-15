use rand::Rng;
use servicepoint::{
    Brightness, GlobalBrightnessCommand, Header, Packet, UdpSocketExt,
};
use std::{fmt::Debug, net::UdpSocket};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ZeroBrightnessCommand;

impl Into<Packet> for ZeroBrightnessCommand {
    fn into(self) -> Packet {
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
                command_code: rng.r#gen(),
                a: rng.r#gen(),
                b: rng.r#gen(),
                c: rng.r#gen(),
                d: rng.r#gen(),
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
