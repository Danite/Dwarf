use std::error::Error;
use std::net::UdpSocket;

pub use self::components::*;
pub use self::types::*;

mod components;
mod types;

fn main() -> Result<(), Box<dyn Error>> {
    let qname = "google.com";
    let qtype = QueryType::A;

    let server = ("8.8.8.8", 53);

    let socket = UdpSocket::bind(("0.0.0.0", 43210))?; // arbitrary port

    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;

    socket.send_to(&req_buffer.buffer[0..req_buffer.position], server)?;

    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buffer)?;

    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;
    println!("{:#?}", res_packet);

    for question in res_packet.questions {
        println!("{:#?}", question);
    }

    for answer_record in res_packet.answers {
        println!("{:#?}", answer_record);
    }

    for authority_record in res_packet.authorities {
        println!("{:#?}", authority_record);
    }

    for resource_record in res_packet.resources {
        println!("{:#?}", resource_record);
    }

    Ok(())
}
