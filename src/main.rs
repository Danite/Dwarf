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

    for authorities_record in packet.authorities {
        println!("{:#?}", authorities_record);
    }

    for resources_record in packet.resources {
        println!("{:#?}", resources_record);
    }

    Ok(())
}
