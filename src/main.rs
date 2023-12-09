use::std::net::UdpSocket;
use::bytes::{ Bytes, BytesMut, Buf, BufMut };
use::std::boxed::Box;

struct PacketHeader {
    packet_type: u16,
    frame_id: u16,
    init_id: u32,
    serial_number: u64
}


fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("169.254.60.200:7502")?;
    loop {
    let mut buf = [0; 32];
    let (amt, _src) = socket.recv_from(&mut buf)?;
    let buf = &buf[..amt];

    let mut bytes = BytesMut::with_capacity(1024);
    bytes.put(buf);
    let sensor_pkt: PacketHeader = parse_packet_header(&mut bytes);
    println!("Lidar packet type: {} ", sensor_pkt.packet_type);
    println!("Lidar frame is as follows: {} ", sensor_pkt.frame_id);
    println!("Init ID is: {} ", sensor_pkt.init_id);
    println!("Sensor serial nubmer is: {} ", sensor_pkt.serial_number);
    } 
}

fn parse_packet_header(buf: &mut BytesMut) -> PacketHeader {
    let mut res: PacketHeader = PacketHeader { 
        packet_type: buf.get_u16_le(),
        frame_id: buf.get_u16_le(),
        init_id: 0,
        serial_number: 0,
    };

    let mut temp = buf.get_u64_le();
    let serial = temp;
    println!("This is a test of serial: {}", serial);

    let mut serial: u64 = serial << 40;
    serial = serial >> 40;
    println!("This is a test of temp: {}", temp);
    println!("This is a test of serial: {}", serial);
    res.init_id = (temp as u32) >> 8;

    return res;
}

