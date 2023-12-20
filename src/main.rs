use::std::net::UdpSocket;
use::bytes::{ BytesMut, Buf, BufMut };
use std::fmt;
use clap::Parser;

struct PacketHeader {
    packet_type: u16,
    frame_id: u16,
    init_id: u32,
    serial_number: u64
}

struct MeasurementPacket {
    range: u32,
    reflectivity: u16,
    signal: u16,
    near_ir: u16,
}

impl fmt::Display for PacketHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "[Packet Type: {}, Frame ID: {}, Init ID: {}, Serial number: {}]", 
            self.packet_type,
            self.frame_id,
            self.init_id,
            self.serial_number)
    }
}
impl fmt::Display for MeasurementPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "[range: {}, reflectivity: {}, signal: {}, near-ir: {}]", 
            self.range,
            self.reflectivity,
            self.signal,
            self.near_ir)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    host: String,
    #[arg(long)]
    lidar_port: u32,
    #[arg(long)]
    imu_port: u32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut to_bind: String = args.host;
    to_bind.push(':');
    to_bind.push_str(&args.lidar_port.to_string());
    let socket = UdpSocket::bind(to_bind)?;

    loop {
    //Todo: Buffer per packet type
    let mut buf = [0; 1023];
    let (amt, _src) = socket.recv_from(&mut buf)?;
    let buf = &buf[..amt];
    let mut bytes = BytesMut::with_capacity(1024);
    bytes.put(buf);
    let sensor_pkt: PacketHeader = parse_packet_header(&mut bytes);
    println!("{}", sensor_pkt);

    // Increment buffer, probably something in the crate that allows me to do this.
    bytes.get_u64();
    bytes.get_u64();
    bytes.get_u32();

    let timestamp: u64 = bytes.get_u64_le();
    let measurement_id: u16 = bytes.get_u16_le();

    //Increment packet buffer
    bytes.get_u16_le();

    for _columns in 0 .. 15 {
        let column_pkt = parse_measurement(&mut bytes);
        println!("{}", column_pkt);

    }

    bytes.get_u64();
    bytes.get_u64();
    bytes.get_u64();
    bytes.get_u64();
    } 
}

fn parse_measurement(buf: &mut BytesMut) -> MeasurementPacket{
    let res: MeasurementPacket = MeasurementPacket {
        range: buf.get_u32_le(),
        reflectivity: buf.get_u16_le(),
        signal: buf.get_u16_le(),
        near_ir: buf.get_u16_le()
    };
    buf.get_u16_le();
    res
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

