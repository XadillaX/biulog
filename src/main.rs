use std::io::prelude::*;
use std::net::TcpStream;

fn read_u32(packet: &[u8], offset: usize) -> u32 {
    (packet[offset] as u32) + ((packet[offset + 1]) as u32) +
        ((packet[offset + 2]) as u32) + ((packet[offset + 3]) as u32)
}

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:3306") {
        Ok(v) => v,
        Err(e) => panic!("Err connecting to MySQL: {:?}", e)
    };

    let mut first_packet = [ 0u8; 128 ];
    let read_length = match stream.read(&mut first_packet) {
        Ok(v) => {
            if 0 == v {
                panic!("No length read in the first packet");
            }

            v
        },
        Err(e) => panic!("Err read the first packet: {:?}", e)
    };

    println!("Packet Read: {}", read_length);

    let content_length =
        (first_packet[0] as u32) + ((first_packet[1] as u32) << 8u32) + ((first_packet[2] as u32) << 16u32);
    println!("Content Length: {}", content_length);
    println!("Packet Number: {}", first_packet[3]);
    println!("Protocol Version: {}", first_packet[4]);

    let mut offset;
    let mut version_info = String::new();
    {
        offset = 5;
        for i in offset..offset + 31 {
            if 0 == first_packet[i] {
                break;
            }

            version_info.push(first_packet[i] as char);
        }
    }
    println!("Protocol Information: {}", version_info);

    let server_thread_id = read_u32(&first_packet, 32);
    println!("Server Thread ID: {}", server_thread_id);
}
