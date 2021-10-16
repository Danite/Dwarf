use crate::components::BytePacketBuffer;
use crate::types::ResultCode;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16,

    pub recursion_desired: bool,
    pub truncated_message: bool,
    pub authoritative_answer: bool,
    pub opcode: u8,
    pub response: bool,

    pub rescode: ResultCode,
    pub checking_disabled: bool,
    pub authed_data: bool,
    pub z: bool,
    pub recursion_available: bool,

    pub questions: u16,
    pub answers: u16,
    pub authoritative_entries: u16,
    pub resource_entries: u16,
}

impl DnsHeader {
    pub fn new() -> Self {
        Self {
            id: 0,

            recursion_desired: false,
            truncated_message: false,
            authoritative_answer: false,
            opcode: 0,
            response: false,

            rescode: ResultCode::NOERROR,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,

            questions: 0,
            answers: 0,
            authoritative_entries: 0,
            resource_entries: 0,
        }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), Box<dyn Error>> {
        self.id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;
        let next_byte = (flags >> 8) as u8;
        let matched8bits = (flags & 0xFF) as u8;

        self.recursion_desired = (next_byte & (1 << 0)) > 0;
        self.truncated_message = (next_byte & (1 << 1)) > 0;
        self.authoritative_answer = (next_byte & (1 << 2)) > 0;
        self.opcode = (next_byte >> 3) & 0x0F;
        self.response = (next_byte & (1 << 7)) > 0;

        self.rescode = ResultCode::from_num(matched8bits & 0x0F);
        self.checking_disabled = (matched8bits & (1 << 4)) > 0;
        self.authed_data = (matched8bits & (1 << 5)) > 0;
        self.z = (matched8bits & (1 << 6)) > 0;
        self.recursion_available = (matched8bits & (1 << 7)) > 0;

        self.questions = buffer.read_u16()?;
        self.answers = buffer.read_u16()?;
        self.authoritative_entries = buffer.read_u16()?;
        self.resource_entries = buffer.read_u16()?;

        Ok(())
    }
}
