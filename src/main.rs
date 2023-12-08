use::std::net::UdpSocket;
use::std::ptr;

struct PacketHeader {
    packet_type: u16,
    frame_id: u16,
    init_id: u32,
    serial_number: u64
}


fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("10.0.0.95:7502")?;
    let mut buf = [0;32];

    loop {
    let (amt, _src) = socket.recv_from(&mut buf)?;
    println!("Something from the sensor: {} from IP:{}", amt, _src);

    let buf = &mut buf[..amt];
    let sensor_pkt: PacketHeader = parse_packet_header(buf);
    } 
}

fn parse_packet_header(buf: &[u8]) -> PacketHeader {

    let res: PacketHeader = PacketHeader { 
        packet_type: u16::from_le_bytes(buf[0..2]),
        frame_id: 0,
        init_id: 0,
        serial_number: 0,
    };

    return res;
}

fn extract_bits<T> (data: <T>, start: usize, end: usize) -> <T>{
    let bitmask = ((1 << (end_bit - start_bit)) - 1) << start_bit; 
    println!("The following bit mask is: {}", bitmask);
}

