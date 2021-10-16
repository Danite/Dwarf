use std::error::Error;
use std::fs::File;
use std::io::Read;

pub use self::components::*;

mod components;
mod types;

fn main() -> Result<(), Box<dyn Error>> {
    let mut response_packet_file = File::open("./packets/response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();

    response_packet_file.read(&mut buffer.buffer)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for question in packet.questions {
        println!("{:#?}", question);
    }

    for answer_record in packet.answers {
        println!("{:#?}", answer_record);
    }

    for authority_record in packet.authorities {
        println!("{:#?}", authority_record);
    }

    for resource_record in packet.resources {
        println!("{:#?}", resource_record);
    }

    Ok(())
}
