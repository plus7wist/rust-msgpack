use crate::codes;
use crate::utils;

pub struct Encoder {
    buf: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder { buf: Vec::new() }
    }

    fn write_code(&mut self, c: codes::Code) -> Result<(), Box<dyn std::error::Error>> {
        self.buf.push(c);
        Ok(())
    }

    fn write_byte(&mut self, c: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.buf.push(c);
        Ok(())
    }

    fn write(&mut self, b: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.buf.extend(b.iter().cloned());
        Ok(())
    }

    fn write_string(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.buf.extend(s.as_bytes().iter().cloned());
        Ok(())
    }

    fn write1(&mut self, c: codes::Code, n: u8) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf: [u8; 2] = [0; 2];
        buf[0] = c;
        buf[1] = n;
        self.write(&buf)
    }

    fn write2(&mut self, c: codes::Code, n: u16) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf: [u8; 3] = [0; 3];
        buf[0] = c;
        buf[1] = (n >> 8) as u8;
        buf[2] = n as u8;
        self.write(&buf)
    }

    fn write4(&mut self, c: codes::Code, n: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf: [u8; 5] = [0; 5];
        buf[0] = c;
        buf[1] = (n >> 24) as u8;
        buf[2] = (n >> 16) as u8;
        buf[3] = (n >> 8) as u8;
        buf[4] = n as u8;
        self.write(&buf)
    }

    fn write8(&mut self, c: codes::Code, n: u64) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0] = c;
        buf[1] = (n >> 56) as u8;
        buf[2] = (n >> 48) as u8;
        buf[3] = (n >> 40) as u8;
        buf[4] = (n >> 32) as u8;
        buf[5] = (n >> 24) as u8;
        buf[6] = (n >> 16) as u8;
        buf[7] = (n >> 8) as u8;
        buf[8] = n as u8;
        self.write(&buf)
    }

    pub fn encode_nil(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.write_code(codes::NIL)
    }

    pub fn encode_bool(&mut self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        if value {
            return self.write_code(codes::TRUE);
        }
        self.write_code(codes::FALSE)
    }
}

impl Encoder {
    pub fn encode_string(&mut self, v: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(error) = self.encode_str_len(v.len() as i32) {
            return Err(error);
        }
        self.write_string(v)
    }

    fn encode_str_len(&mut self, l: i32) -> Result<(), Box<dyn std::error::Error>> {
        if l < 32 {
            return self.write_code(codes::FIXED_STR_LOW | (l as codes::Code));
        }
        if l < 256 {
            return self.write1(codes::STR_8, l as u8);
        }
        if l < 65536 {
            return self.write2(codes::STR_16, l as u16);
        }
        self.write4(codes::STR_32, l as u32)
    }
}

impl Encoder {
    pub fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(error) = self.encode_bytes_len(v.len() as i32) {
            return Err(error);
        }
        self.write(v)
    }

    fn encode_bytes_len(&mut self, l: i32) -> Result<(), Box<dyn std::error::Error>> {
        if l < 256 {
            return self.write1(codes::BIN_8, l as u8);
        }
        if l < 65536 {
            return self.write2(codes::BIN_16, l as u16);
        }
        self.write4(codes::BIN_32, l as u32)
    }
}

impl Encoder {
    pub fn encode_int(&mut self, v: i64) -> Result<(), Box<dyn std::error::Error>> {
        if v >= 0 {
            return self.encode_uint(v as u64);
        }
        if v >= ((v as i8) as i64) {
            return self.write_byte(v as u8);
        }
        if v >= (std::i8::MIN as i64) {
            return self.write1(codes::INT_8, v as u8);
        }
        if v >= (std::i16::MIN as i64) {
            return self.write2(codes::INT_16, v as u16);
        }
        if v >= (std::i32::MIN as i64) {
            return self.write4(codes::INT_32, v as u32);
        }
        self.write8(codes::INT_64, v as u64)
    }

    pub fn encode_uint(&mut self, v: u64) -> Result<(), Box<dyn std::error::Error>> {
        if v <= (std::i8::MAX as u64) {
            return self.write_byte(v as u8);
        }
        if v <= (std::u8::MAX as u64) {
            return self.write1(codes::UINT_8, v as u8);
        }
        if v <= (std::u16::MAX as u64) {
            return self.write2(codes::UINT_16, v as u16);
        }
        if v <= (std::u32::MAX as u64) {
            return self.write4(codes::UINT_32, v as u32);
        }
        self.write8(codes::UINT_64, v)
    }
}

impl Encoder {
    pub fn encode_float32(&mut self, f: f32) -> Result<(), Box<dyn std::error::Error>> {
        self.write4(codes::FLOAT_32, utils::float32bits(f))
    }

    pub fn encode_float64(&mut self, f: f64) -> Result<(), Box<dyn std::error::Error>> {
        self.write8(codes::FLOAT_64, utils::float64bits(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty_string() {
        let mut enc = Encoder::new();
        enc.encode_string("").unwrap();
        assert_eq!(&enc.buf, &[0xa0]);
    }
}
