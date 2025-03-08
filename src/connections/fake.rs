use crate::{Connection, Packet};
use log::debug;

#[derive(Debug)]
/// A fake connection for testing that does not actually send anything.
pub struct FakeConnection;

impl Connection for FakeConnection {
    // TODO: () does not implement Error+Debug, some placeholder is needed
    type Error = std::io::Error;

    fn send(&self, packet: impl Into<Packet>) -> Result<(), Self::Error> {
        let data: Vec<u8> = packet.into().into();
        debug!("Sending fake packet: {data:?}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Packet;

    #[test]
    fn send_fake() {
        let data: &[u8] = &[0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let packet = Packet::try_from(data).unwrap();
        FakeConnection.send(packet).unwrap()
    }
}
