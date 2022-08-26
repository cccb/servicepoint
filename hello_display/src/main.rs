use anyhow::Result;

use std::net::UdpSocket;

fn main() -> Result<()> {
    println!("Sending hello display...");

    let socket = UdpSocket::bind("0.0.0.0:17382")?;


    for frame in frames {
    socket.send_to(frame, "
        }

    Ok(())
}
