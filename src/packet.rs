pub struct Header(pub u16, pub u16, pub u16, pub u16, pub u16);

pub type Payload = Vec<u8>;

pub struct Packet(pub Header, pub Payload);

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        let Packet(Header(mode, a, b, c, d), payload) = self;

        let mut packet = vec!(0u8; 10 + payload.len());
        packet[0..=1].copy_from_slice(&u16::to_be_bytes(mode));
        packet[2..=3].copy_from_slice(&u16::to_be_bytes(a));
        packet[4..=5].copy_from_slice(&u16::to_be_bytes(b));
        packet[6..=7].copy_from_slice(&u16::to_be_bytes(c));
        packet[8..=9].copy_from_slice(&u16::to_be_bytes(d));

        packet[10..].copy_from_slice(&*payload);

        return packet;
    }
}

fn u16_from_be_slice(slice: &[u8]) -> u16 {
    let mut bytes = [0u8; 2];
    bytes[0] = slice[0];
    bytes[1] = slice[1];
    u16::from_be_bytes(bytes)
}

impl From<Vec<u8>> for Packet {
    fn from(value: Vec<u8>) -> Self {
        let mode = u16_from_be_slice(&value[0..=1]);
        let a = u16_from_be_slice(&value[2..=3]);
        let b = u16_from_be_slice(&value[4..=5]);
        let c = u16_from_be_slice(&value[6..=7]);
        let d = u16_from_be_slice(&value[8..=9]);
        let payload = value[10..].to_vec();

        Packet(Header(mode, a, b, c, d), payload)
    }
}