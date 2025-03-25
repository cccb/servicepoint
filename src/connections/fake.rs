use crate::{Connection, Packet, SendError};
use log::debug;
use std::{convert::Infallible, error::Error, fmt::Debug};

#[derive(Debug)]
/// A fake connection for testing that does not actually send anything.
pub struct FakeConnection;

impl Connection for FakeConnection {
    type TransportError = Infallible;

    fn send<P: TryInto<Packet>>(
        &self,
        packet: P,
    ) -> Result<
        (),
        SendError<<P as TryInto<Packet>>::Error, Self::TransportError>,
    >
    where
        <P as TryInto<Packet>>::Error: Error + Debug,
    {
        let data: Vec<u8> = packet
            .try_into()
            .map(Into::<Vec<u8>>::into)
            .map_err(SendError::IntoPacket)?;
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
        FakeConnection.send(packet).unwrap();
    }
}
