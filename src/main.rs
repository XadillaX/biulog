use std::io::prelude::*;
use std::net::TcpStream;

fn read_u32(packet: &[u8], offset: usize) -> u32 {
    (packet[offset] as u32) + ((packet[offset + 1]) as u32) +
        ((packet[offset + 2]) as u32) + ((packet[offset + 3]) as u32)
}

fn read_8bytes(packet: &[u8], offset: usize) -> [u8;8] {
    let mut _packet = [0u8; 8];
    for i in 0..8 {
        _packet[i] = packet[i + offset];
    }

    _packet
}

fn read_n_bytes(packet: &[u8], offset: usize, size: usize) -> Vec<u8> {
    let mut vec = vec![];
    for i in 0..size {
        vec.push(packet[i + offset]);
    }

    vec
}

fn read_u16(packet: &[u8], offset: usize) -> u16 {
    (packet[offset] as u16) + ((packet[offset + 1]) as u16)
}

fn vec_to_string(vec: Vec<u8>) -> String {
    let mut str = String::new();
    for i in 0..vec.len() {
        if 0 == vec[i] {
            break;
        }

        str.push(vec[i] as char);
    }

    str
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
        for i in offset..offset + 32 {
            if 0 == first_packet[i] {
                offset = i + 1;
                break;
            }

            version_info.push(first_packet[i] as char);
        }
    }
    println!("Protocol Information: {}", version_info);

    let server_thread_id = read_u32(&first_packet, offset);
    println!("Server Thread ID: {}", server_thread_id);

    offset += 4;
    let challenge_random_number1 = read_8bytes(&first_packet, offset);
    println!("Challenge Random Number 1: {:?}", challenge_random_number1);

    offset += 8 + 1;
    let server_capabilities_identified = read_u16(&first_packet, offset);
    println!("Server Capabilities Identified: {}", server_capabilities_identified);

    offset += 2;
    let char_code = first_packet[offset];
    println!("Char Code: {}", char_code);

    offset += 1;
    let server_status = read_u16(&first_packet, offset);
    println!("Server Status: {}", server_status);

    offset += 2;
    let server_capabilities_identified_high = read_u16(&first_packet, offset);
    println!("Server Capabilities Identified High: {}", server_capabilities_identified_high);

    offset += 2;
    let challenge_length = first_packet[offset];
    println!("Challenge Length: {}", challenge_length);

    offset += 1;
    let fill_value = read_n_bytes(&first_packet, offset, 10);
    println!("Fill Value: {:?}", fill_value);
    
    offset += 10;

    let mut challenge_random_number2 : Vec<u8> = vec![];
    {
        for i in offset..read_length as usize {
            if 0 == first_packet[i] {
                offset = i + 1;
                break;
            }

            challenge_random_number2.push(first_packet[i]);
        }
    }
    println!("Challenge Random Number 2: {:?}", challenge_random_number2);

    println!("Left: {:?}", vec_to_string(read_n_bytes(&first_packet, offset, read_length - offset)));
}
