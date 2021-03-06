use std::error::Error;
use std::net::{Ipv4Addr, UdpSocket};

pub use self::components::*;
pub use self::types::*;

mod components;
mod types;

fn lookup(
    qname: &str,
    qtype: QueryType,
    server: (Ipv4Addr, u16),
) -> Result<DnsPacket, Box<dyn Error>> {
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

    DnsPacket::from_buffer(&mut res_buffer)
}

// TODO: refactor
fn recursive_lookup(qname: &str, qtype: QueryType) -> Result<DnsPacket, Box<dyn Error>> {
    // *a.root-servers.net*
    let mut ns = "198.41.0.4".parse::<Ipv4Addr>().unwrap();

    loop {
        println!("attempting lookup of {:?} {} with ns {}", qtype, qname, ns);

        let ns_copy = ns;

        let server = (ns_copy, 53);
        let response = lookup(qname, qtype, server)?;

        if !response.answers.is_empty() && response.header.rescode == ResultCode::NOERROR {
            return Ok(response);
        }

        if response.header.rescode == ResultCode::NXDOMAIN {
            return Ok(response);
        }

        if let Some(new_ns) = response.get_resolved_ns(qname) {
            ns = new_ns;
            continue;
        }

        let new_ns_name = match response.get_unresolved_ns(qname) {
            Some(ns) => ns,
            None => return Ok(response),
        };

        let recursive_response = recursive_lookup(&new_ns_name, QueryType::A)?;

        if let Some(new_ns) = recursive_response.get_random_a_record() {
            ns = new_ns;
        } else {
            return Ok(response);
        }
    }
}

fn handle_query(socket: &UdpSocket) -> Result<(), Box<dyn Error>> {
    let mut req_buffer = BytePacketBuffer::new();

    let (_, src) = socket.recv_from(&mut req_buffer.buffer)?;

    let mut request = DnsPacket::from_buffer(&mut req_buffer)?;

    let mut packet = DnsPacket::new();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;

    if let Some(question) = request.questions.pop() {
        println!("Received query: {:?}", question);

        if let Ok(result) = recursive_lookup(&question.name, question.qtype) {
            packet.questions.push(question.clone());
            packet.header.rescode = result.header.rescode;

            for answer_record in result.answers {
                println!("Answer: {:#?}", answer_record);
                packet.answers.push(answer_record);
            }

            for authority_record in result.authorities {
                println!("Authority: {:#?}", authority_record);
                packet.authorities.push(authority_record);
            }

            for resource_record in result.resources {
                println!("Resource: {:#?}", resource_record);
                packet.resources.push(resource_record);
            }
        } else {
            packet.header.rescode = ResultCode::SERVFAIL
        }
    } else {
        packet.header.rescode = ResultCode::FORMERR;
    }

    let mut res_buffer = BytePacketBuffer::new();
    packet.write(&mut res_buffer)?;

    let length = res_buffer.position();
    let data = res_buffer.get_range(0, length)?;

    socket.send_to(data, src)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;

    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(error) => eprintln!("An error ocurred: {}", error),
        }
    }
}
