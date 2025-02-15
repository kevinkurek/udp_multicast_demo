use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

// This is note a real SIP message, but a simplified version for demonstration purposes
// This is not used within the project, but is a standalone example

struct SipMessage {
    timestamp: u32,
    security_id: u32,
    message_type: u8,
    price: f32,
    size: u32
}

fn parse_sip_binary(data: &[u8]) -> SipMessage {
    let mut cursor = Cursor::new(data);

    let timestamp = cursor.read_u32::<BigEndian>().unwrap();
    let security_id = cursor.read_u32::<BigEndian>().unwrap();
    let message_type = cursor.read_u8().unwrap();
    let price = cursor.read_u32::<BigEndian>().unwrap() as f32 / 100.0;
    let size = cursor.read_u32::<BigEndian>().unwrap();

    SipMessage{
        timestamp,
        security_id,
        message_type,
        price,
        size
    }
}

pub fn run() {
    let binary_data: Vec<u8> = vec![
        0x61, 0xAB, 0xCD, 0x78, // Timestamp
        0x00, 0x02, 0xB6, 0x7A, // Security ID
        0x01,                   // Message Type (Trade)
        0x00, 0x00, 0x2A, 0x2E, // Price (107.98)
        0x00, 0x00, 0x00, 0x3E, // Size (62 shares)
    ];

    let msg = parse_sip_binary(&binary_data);
    println!("Timestamp: {}, SecurityID: {}, Type: {}, Price: {:.2}, Size: {}",
            msg.timestamp, msg.security_id, msg.message_type, msg.price, msg.size);
}

fn main() {
    run();
}