use std::error::Error;
use std::fs::File;
use std::io::Read;

pub use self::components::*;

mod components;

fn main() -> Result<(), Box<dyn Error>> {
    let mut response_packet = File::open("../packets/response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();

    response_packet.read(&mut buffer.buffer)?;

    Ok(())
}
