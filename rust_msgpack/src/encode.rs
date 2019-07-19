use crate::binary;
use crate::codes;
use crate::error::Error as RMError;
use crate::time;
use crate::utils;
use std::time::SystemTime;

pub struct Encoder {
    pub buf: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder { buf: Vec::new() }
    }

    fn write_code(&mut self, c: codes::Code) -> Result<(), RMError> {
        self.buf.push(c);
        Ok(())
    }

    fn write_byte(&mut self, c: u8) -> Result<(), RMError> {
        self.buf.push(c);
        Ok(())
    }

    fn write(&mut self, b: &[u8]) -> Result<(), RMError> {
        self.buf.extend(b.iter().cloned());
        Ok(())
    }

    fn write_string(&mut self, s: &str) -> Result<(), RMError> {
        self.buf.extend(s.as_bytes().iter().cloned());
        Ok(())
    }

    fn write1(&mut self, c: codes::Code, n: u8) -> Result<(), RMError> {
        let mut buf: [u8; 2] = [0; 2];
        buf[0] = c;
        buf[1] = n;
        self.write(&buf)
    }

    fn write2(&mut self, c: codes::Code, n: u16) -> Result<(), RMError> {
        let mut buf: [u8; 3] = [0; 3];
        buf[0] = c;
        binary::BigEndian::put_uint16(&mut buf[1..3], n);
        self.write(&buf)
    }

    fn write4(&mut self, c: codes::Code, n: u32) -> Result<(), RMError> {
        let mut buf: [u8; 5] = [0; 5];
        buf[0] = c;
        binary::BigEndian::put_uint32(&mut buf[1..5], n);
        self.write(&buf)
    }

    fn write8(&mut self, c: codes::Code, n: u64) -> Result<(), RMError> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0] = c;
        binary::BigEndian::put_uint64(&mut buf[1..9], n);
        self.write(&buf)
    }

    pub fn encode_nil(&mut self) -> Result<(), RMError> {
        self.write_code(codes::NIL)
    }

    pub fn encode_bool(&mut self, value: bool) -> Result<(), RMError> {
        if value {
            return self.write_code(codes::TRUE);
        }
        self.write_code(codes::FALSE)
    }
}

impl Encoder {
    pub fn encode_string(&mut self, v: &str) -> Result<(), RMError> {
        if let Err(error) = self.encode_str_len(v.len() as i32) {
            return Err(error);
        }
        self.write_string(v)
    }

    fn encode_str_len(&mut self, l: i32) -> Result<(), RMError> {
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
    pub fn encode_bytes(&mut self, v: &[u8]) -> Result<(), RMError> {
        if let Err(error) = self.encode_bytes_len(v.len() as i32) {
            return Err(error);
        }
        self.write(v)
    }

    fn encode_bytes_len(&mut self, l: i32) -> Result<(), RMError> {
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
    pub fn encode_int(&mut self, v: i64) -> Result<(), RMError> {
        if v >= 0 {
            return self.encode_uint(v as u64);
        }
        if v >= ((codes::NEG_FIXED_NUM_LOW as i8) as i64) {
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

    pub fn encode_uint(&mut self, v: u64) -> Result<(), RMError> {
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
    pub fn encode_float32(&mut self, f: f32) -> Result<(), RMError> {
        self.write4(codes::FLOAT_32, utils::float32bits(f))
    }

    pub fn encode_float64(&mut self, f: f64) -> Result<(), RMError> {
        self.write8(codes::FLOAT_64, utils::float64bits(f))
    }
}

impl Encoder {
    pub fn encode_array_len(&mut self, l: i32) -> Result<(), RMError> {
        if l < 16 {
            return self.write_code(codes::FIXED_ARRAY_LOW | (l as codes::Code));
        }
        if l < 65536 {
            return self.write2(codes::ARRAY_16, l as u16);
        }
        self.write4(codes::ARRAY_32, l as u32)
    }
}

impl Encoder {
    pub fn encode_map_len(&mut self, l: i32) -> Result<(), RMError> {
        if l < 16 {
            return self.write_code(codes::FIXED_MAP_LOW | (l as codes::Code));
        }
        if l < 65536 {
            return self.write2(codes::MAP_16, l as u16);
        }
        self.write4(codes::MAP_32, l as u32)
    }
}

impl Encoder {
    fn encode_ext_len(&mut self, l: i32) -> Result<(), RMError> {
        if l == 1 {
            return self.write_code(codes::FIX_EXT_1);
        }
        if l == 2 {
            return self.write_code(codes::FIX_EXT_2);
        }
        if l == 4 {
            return self.write_code(codes::FIX_EXT_4);
        }
        if l == 8 {
            return self.write_code(codes::FIX_EXT_8);
        }
        if l == 16 {
            return self.write_code(codes::FIX_EXT_16);
        }
        if l < 256 {
            return self.write1(codes::EXT_8, l as u8);
        }
        if l < 65536 {
            return self.write2(codes::EXT_16, l as u16);
        }
        self.write4(codes::EXT_32, l as u32)
    }
}

impl Encoder {
    pub fn encode_time(&mut self, t: SystemTime) -> Result<(), RMError> {
        let b = time::encode_time(t);
        if let Err(error) = self.encode_ext_len(b.len() as i32) {
            return Err(error);
        }

        const TIME_EXT_ID: i32 = -1;
        if let Err(error) = self.write_byte(TIME_EXT_ID as u8) {
            return Err(error);
        }

        self.write(&b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_nil() {
        let mut enc = Encoder::new();
        enc.encode_nil().unwrap();
        assert_eq!(&enc.buf, &[0xc0]);
    }

    #[test]
    fn test_encode_bool() {
        let mut enc = Encoder::new();
        enc.encode_bool(true).unwrap();
        assert_eq!(&enc.buf, &[0xc3]);

        let mut enc = Encoder::new();
        enc.encode_bool(false).unwrap();
        assert_eq!(&enc.buf, &[0xc2]);

        let mut enc = Encoder::new();
        enc.encode_bool(true).unwrap();
        enc.encode_bool(false).unwrap();
        assert_eq!(&enc.buf, &[0xc3, 0xc2]);

        let mut enc = Encoder::new();
        enc.encode_bool(false).unwrap();
        enc.encode_bool(true).unwrap();
        assert_eq!(&enc.buf, &[0xc2, 0xc3]);
    }

    #[test]
    fn test_encode_empty_string() {
        let mut enc = Encoder::new();
        enc.encode_string("").unwrap();
        assert_eq!(&enc.buf, &[0xa0]);
    }

    #[test]
    fn test_encode_fix_string() {
        let mut enc = Encoder::new();
        enc.encode_string("hello").unwrap();
        assert_eq!(&enc.buf, &[0xA5, 0x68, 0x65, 0x6C, 0x6C, 0x6F]);
    }

    #[test]
    fn test_encode_string_1() {
        let mut enc = Encoder::new();
        enc.encode_string("RYUGJGWSLOHAOPHMQNWVBWIYEVOXDTKU")
            .unwrap();
        let expectoutput = vec![
            0xD9, 0x20, 0x52, 0x59, 0x55, 0x47, 0x4A, 0x47, 0x57, 0x53, 0x4C, 0x4F, 0x48, 0x41,
            0x4F, 0x50, 0x48, 0x4D, 0x51, 0x4E, 0x57, 0x56, 0x42, 0x57, 0x49, 0x59, 0x45, 0x56,
            0x4F, 0x58, 0x44, 0x54, 0x4B, 0x55,
        ];
        assert_eq!(&enc.buf, &expectoutput);
    }

    #[test]
    fn test_encode_bytes_1() {
        let mut enc = Encoder::new();
        let input = vec![b'a', b'b'];
        enc.encode_bytes(&input[..]).unwrap();
        let expectoutput = vec![0xC4, 0x02, 0x61, 0x62];
        assert_eq!(&enc.buf, &expectoutput);
    }

    #[test]
    fn test_encode_integer() {
        let mut enc = Encoder::new();
        let input: i64 = -214748364700;
        enc.encode_int(input).unwrap();
        let expectoutput = vec![0xD3, 0xFF, 0xFF, 0xFF, 0xCE, 0x00, 0x00, 0x00, 0x64];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = -70000;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0xD2, 0xFF, 0xFE, 0xEE, 0x90];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = -300;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0xD1, 0xFE, 0xD4];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = -200;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0xD1, 0xFF, 0x38];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = -100;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0xD0, 0x9C];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = -1;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0xFF];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = 0;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0x00];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = 1;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0x01];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = 100;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0x64];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = 200;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0xcc, 0xc8];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = 300;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0xcd, 0x01, 0x2c];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input = 70000;
        enc.encode_int(input as i64).unwrap();
        let expectoutput = vec![0xCE, 0x00, 0x01, 0x11, 0x70];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input: i64 = 214748364700;
        enc.encode_int(input).unwrap();
        let expectoutput = vec![0xCF, 0x00, 0x00, 0x00, 0x31, 0xFF, 0xFF, 0xFF, 0x9C];
        assert_eq!(&enc.buf, &expectoutput);
    }

    #[test]
    fn test_encode_float() {
        let mut enc = Encoder::new();
        let input: f32 = 1.23456;
        enc.encode_float32(input).unwrap();
        let expectoutput = vec![0xCA, 0x3F, 0x9E, 0x06, 0x10];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input: f64 = 1.23456;
        enc.encode_float64(input).unwrap();
        let expectoutput = vec![0xCB, 0x3F, 0xF3, 0xC0, 0xC1, 0xFC, 0x8F, 0x32, 0x38];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input: f32 = 1.0 / 3.0;
        enc.encode_float32(input).unwrap();
        let expectoutput = vec![0xCA, 0x3E, 0xAA, 0xAA, 0xAB];
        assert_eq!(&enc.buf, &expectoutput);

        let mut enc = Encoder::new();
        let input: f64 = 1.0 / 3.0;
        enc.encode_float64(input).unwrap();
        let expectoutput = vec![0xCB, 0x3F, 0xD5, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
        assert_eq!(&enc.buf, &expectoutput);
    }

    #[test]
    fn test_encode_array() {
        let mut enc = Encoder::new();
        enc.encode_array_len(4).unwrap();
        enc.encode_int(1).unwrap();
        enc.encode_int(2).unwrap();
        enc.encode_int(3).unwrap();
        enc.encode_int(4).unwrap();
        let expectoutput = vec![0x94, 0x01, 0x02, 0x03, 0x04];
        assert_eq!(&enc.buf, &expectoutput);
    }

    #[test]
    fn test_encode_map() {
        let mut enc = Encoder::new();
        enc.encode_map_len(1).unwrap();
        enc.encode_string("name").unwrap();
        enc.encode_string("huangjian").unwrap();
        let expectoutput = vec![
            0x81, 0xA4, 0x6E, 0x61, 0x6D, 0x65, 0xA9, 0x68, 0x75, 0x61, 0x6E, 0x67, 0x6A, 0x69,
            0x61, 0x6E,
        ];
        assert_eq!(&enc.buf, &expectoutput);
    }
}
