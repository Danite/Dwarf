use std::error::Error;

pub struct BytePacketBuffer {
    pub buffer: [u8; 512],
    pub position: usize,
}

impl BytePacketBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0; 512],
            position: 0,
        }
    }

    fn position(&self) -> usize {
        self.position
    }

    fn step(&mut self, step: usize) -> Result<(), Box<dyn Error>> {
        self.position += step;

        Ok(())
    }

    fn seek(&mut self, position: usize) -> Result<(), Box<dyn Error>> {
        self.position = position;

        Ok(())
    }

    fn read(&mut self) -> Result<u8, Box<dyn Error>> {
        if self.position >= 512 {
            return Err("Emd of buffer".into());
        }

        let result = self.buffer[self.position];
        self.position += 1;

        Ok(result)
    }

    fn get(&self, position: usize) -> Result<u8, Box<dyn Error>> {
        if self.position >= 512 {
            return Err("Emd of buffer".into());
        }

        Ok(self.buffer[position])
    }

    fn get_range(&self, start: usize, end: usize) -> Result<&[u8], Box<dyn Error>> {
        if self.position >= 512 {
            return Err("Emd of buffer".into());
        }

        Ok(&self.buffer[start..start + end as usize])
    }

    pub fn read_u16(&mut self) -> Result<u16, Box<dyn Error>> {
        let result = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(result)
    }

    fn read_u32(&mut self) -> Result<u32, Box<dyn Error>> {
        let result = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        Ok(result)
    }

    fn read_qname(&mut self, outstr: &mut String) -> Result<(), Box<dyn Error>> {
        let mut local_position = self.position();

        let mut has_jumped = false;
        let max_jumps = 5;
        let mut jumps_done = 0;

        let mut delimeter = "";

        loop {
            if jumps_done > max_jumps {
                return Err(format!("Limit of {} jumps exceeded", max_jumps).into());
            }

            let length = self.get(local_position)?;

            if (length & 0xC0) == 0xC0 {
                if !has_jumped {
                    self.seek(local_position + 2)?;
                }

                let next_byte = self.get(local_position + 1)? as u16;
                let offset = (((length as u16) ^ 0xC0) << 8) | next_byte;
                local_position = offset as usize;

                has_jumped = true;
                jumps_done += 1;

                continue;
            }

            local_position += 1;

            if length == 0 {
                break;
            }

            outstr.push_str(delimeter);

            let str_buffer = self.get_range(local_position, length as usize)?;
            outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

            delimeter = ".";

            local_position += length as usize;
        }

        if !has_jumped {
            self.seek(local_position)?;
        }

        Ok(())
    }
}
