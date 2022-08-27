use anyhow::Result;
use std::net::UdpSocket;

use super::commands::Command;
use crate::protocol::Data;

pub struct Display {
    addr: String,
    socket: UdpSocket,
}

impl Display {
    /// Open a new UDP socket and create a display instance
    pub fn open(addr: String) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self {
            addr: addr,
            socket: socket,
        })
    }

    /// Send a command to the display
    pub fn send(&self, cmd: Command) -> Result<()> {
        let data: Data = cmd.into();
        for frame in data {
            self.socket
                .send_to(frame.as_slice(), self.addr.clone().as_str())?;
        }
        Ok(())
    }
}
