pub struct Header(pub u16, pub u16, pub u16, pub u16, pub u16);

pub type Payload = Vec<u8>;

pub struct Packet(pub Header, pub Payload);

impl Packet {
    pub fn to_bytes(&self) -> Vec<u8> {
        let Packet(Header(mode, a, b, c, d), payload) = self;

        let mut packet = vec!(0u8; 10 + payload.len());
        packet[0..=1].copy_from_slice(&u16::to_be_bytes(*mode));
        packet[2..=3].copy_from_slice(&u16::to_be_bytes(*a));
        packet[4..=5].copy_from_slice(&u16::to_be_bytes(*b));
        packet[6..=7].copy_from_slice(&u16::to_be_bytes(*c));
        packet[8..=9].copy_from_slice(&u16::to_be_bytes(*d));

        packet[10..].copy_from_slice(&*payload);

        return packet;
    }
}