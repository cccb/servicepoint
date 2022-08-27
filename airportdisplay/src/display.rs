
use anyhow::Result;
use std::net::UdpSocket;

use crate::display::commands::Command;
use crate::display::protocol::Data;

pub struct Display {
    addr: String,
    socket: UdpSocket,
}


impl Display {
    pub fn open(addr: String) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:17382")?;
        Ok(Self{
            addr: addr,
            socket: socket,
        })
    }

    pub fn send(&self, cmd: Command) -> Result<()> {
        let data: Data = cmd.into();
        for frame in data {
            println!("sending payload: {:?}", &frame);
            self.socket.send_to(frame.as_slice(), self.addr.clone().as_str())?;
        }
        Ok(())
    }
}

