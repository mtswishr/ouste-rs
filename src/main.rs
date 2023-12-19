use::std::net::UdpSocket;
use::bytes::{ BytesMut, Buf, BufMut };
use std::fmt;

struct PacketHeader {
    packet_type: u16,
    frame_id: u16,
    init_id: u32,
    serial_number: u64
}

impl fmt::Display for PacketHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "Packer header is as follow:\nPacket Type: {}, Frame ID: {}, Init ID: {}, Serial number: {}", 
            self.packet_type,
            self.frame_id,
            self.init_id,
            self.serial_number)
    }
}


fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("169.254.58.45:7502")?;
    loop {
    let mut buf = [0; 32];
    let (amt, _src) = socket.recv_from(&mut buf)?;
    let buf = &buf[..amt];

    let mut bytes = BytesMut::with_capacity(1024);
    bytes.put(buf);
    let sensor_pkt: PacketHeader = parse_packet_header(&mut bytes);
    println!("Print sensor packet: {}", sensor_pkt)
    } 
}

fn parse_packet_header(buf: &mut BytesMut) -> PacketHeader {
    let mut res: PacketHeader = PacketHeader { 
        packet_type: buf.get_u16_le(),
        frame_id: buf.get_u16_le(),
        init_id: 0,
        serial_number: 0,
    };

    let temp = buf.get_u64_le();

    let mut lower: u64 = temp << 40;
    lower = lower >> 40;
    let upper: u64 = temp >> 24;
    res.init_id = lower as u32;
    res.serial_number = upper;

    return res;
}

