use crate::binary;
use crate::bytes;
use crate::codes;
use crate::error::Error as RMError;
use crate::utils;
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
use value::value::Value;

pub struct Decoder<'a> {
    r: bytes::Reader<'a>,
}

impl<'a> Decoder<'a> {
    pub fn new(s: &'a [u8]) -> Decoder {
        Decoder {
            r: bytes::Reader::new(s),
        }
    }

    pub fn read_code(&mut self) -> Result<codes::Code, RMError> {
        let c = self.r.read_byte()?;
        Ok(c as codes::Code)
    }

    pub fn read_byte(&mut self) -> Result<u8, RMError> {
        let c = self.r.read_byte()?;
        Ok(c)
    }

    pub fn read_n(&mut self, n: i32) -> Result<Vec<u8>, RMError> {
        let mut buf: Vec<u8> = vec![0; n as usize];
        let readcount = self.r.read(&mut buf)?;
        if readcount != (n as i64) {
            return Err(RMError::RWNotMatch);
        }
        Ok(buf)
    }

    fn read_uint8(&mut self) -> Result<u8, RMError> {
        self.read_code()
    }

    fn read_int8(&mut self) -> Result<i8, RMError> {
        let c = self.read_uint8()?;
        Ok(c as i8)
    }

    fn read_uint16(&mut self) -> Result<u16, RMError> {
        let b = self.read_n(2)?;
        Ok(binary::BigEndian::uint16(&b))
    }

    fn read_int16(&mut self) -> Result<i16, RMError> {
        let n = self.read_uint16()?;
        Ok(n as i16)
    }

    fn read_uint32(&mut self) -> Result<u32, RMError> {
        let b = self.read_n(4)?;
        Ok(binary::BigEndian::uint32(&b))
    }

    fn read_int32(&mut self) -> Result<i32, RMError> {
        let n = self.read_uint32()?;
        Ok(n as i32)
    }

    fn read_uint64(&mut self) -> Result<u64, RMError> {
        let b = self.read_n(8)?;
        Ok(binary::BigEndian::uint64(&b))
    }

    fn read_int64(&mut self) -> Result<i64, RMError> {
        let n = self.read_uint64()?;
        Ok(n as i64)
    }

    pub fn read_uint(&mut self, c: codes::Code) -> Result<u64, RMError> {
        if c == codes::NIL {
            return Ok(0);
        }
        if codes::is_fixed_num(c) {
            return Ok((c as i8) as u64);
        }
        match c {
            codes::UINT_8 => {
                let n = self.read_uint8()?;
                return Ok(n as u64);
            }
            codes::INT_8 => {
                let n = self.read_int8()?;
                return Ok(n as u64);
            }
            codes::UINT_16 => {
                let n = self.read_uint16()?;
                return Ok(n as u64);
            }
            codes::INT_16 => {
                let n = self.read_int16()?;
                return Ok(n as u64);
            }
            codes::UINT_32 => {
                let n = self.read_uint32()?;
                return Ok(n as u64);
            }
            codes::INT_32 => {
                let n = self.read_int32()?;
                return Ok(n as u64);
            }
            codes::UINT_64 | codes::INT_64 => {
                let n = self.read_uint64()?;
                return Ok(n);
            }
            _ => {
                return Err(RMError::InvalidCode(c));
            }
        }
    }

    pub fn read_int(&mut self, c: codes::Code) -> Result<i64, RMError> {
        if c == codes::NIL {
            return Ok(0);
        }
        if codes::is_fixed_num(c) {
            return Ok((c as i8) as i64);
        }
        match c {
            codes::UINT_8 => {
                let n = self.read_uint8()?;
                return Ok(n as i64);
            }
            codes::INT_8 => {
                let n = self.read_uint8()?;
                return Ok((n as i8) as i64);
            }
            codes::UINT_16 => {
                let n = self.read_uint16()?;
                return Ok(n as i64);
            }
            codes::INT_16 => {
                let n = self.read_uint16()?;
                return Ok((n as i16) as i64);
            }
            codes::UINT_32 => {
                let n = self.read_uint32()?;
                return Ok(n as i64);
            }
            codes::INT_32 => {
                let n = self.read_uint32()?;
                return Ok((n as i32) as i64);
            }
            codes::UINT_64 => {
                let n = self.read_uint64()?;
                return Ok(n as i64);
            }
            codes::INT_64 => {
                let n = self.read_int64()?;
                return Ok(n);
            }
            _ => {
                return Err(RMError::InvalidCode(c));
            }
        }
    }

    fn bytes_len(&mut self, c: codes::Code) -> Result<i32, RMError> {
        if c == codes::NIL {
            return Ok(-1);
        } else if codes::is_fixed_string(c) {
            return Ok((c & codes::FIXED_STR_MASK) as i32);
        } else if c == codes::STR_8 || c == codes::BIN_8 {
            let n = self.read_uint8()?;
            return Ok(n as i32);
        } else if c == codes::STR_16 || c == codes::BIN_16 {
            let n = self.read_uint16()?;
            return Ok(n as i32);
        } else if c == codes::STR_32 || c == codes::BIN_32 {
            let n = self.read_uint32()?;
            return Ok(n as i32);
        }
        Err(RMError::InvalidCode(c))
    }
}

impl<'a> Decoder<'a> {
    pub fn decode_int64(&mut self) -> Result<i64, RMError> {
        let c = self.read_code()?;
        self.read_int(c)
    }

    pub fn decode_int(&mut self) -> Result<i32, RMError> {
        let n = self.decode_int64()?;
        Ok(n as i32)
    }

    pub fn decode_int8(&mut self) -> Result<i8, RMError> {
        let n = self.decode_int64()?;
        Ok(n as i8)
    }

    pub fn decode_int16(&mut self) -> Result<i16, RMError> {
        let n = self.decode_int64()?;
        Ok(n as i16)
    }

    pub fn decode_int32(&mut self) -> Result<i32, RMError> {
        let n = self.decode_int64()?;
        Ok(n as i32)
    }

    pub fn decode_uint64(&mut self) -> Result<u64, RMError> {
        let c = self.read_code()?;
        self.read_uint(c)
    }

    pub fn decode_uint(&mut self) -> Result<u32, RMError> {
        let n = self.decode_uint64()?;
        Ok(n as u32)
    }

    pub fn decode_uint8(&mut self) -> Result<u8, RMError> {
        let n = self.decode_uint64()?;
        Ok(n as u8)
    }

    pub fn decode_uint16(&mut self) -> Result<u16, RMError> {
        let n = self.decode_uint64()?;
        Ok(n as u16)
    }

    pub fn decode_uint32(&mut self) -> Result<u32, RMError> {
        let n = self.decode_uint64()?;
        Ok(n as u32)
    }
}

impl<'a> Decoder<'a> {
    pub fn decode_string(&mut self) -> Result<String, RMError> {
        let c = self.read_code()?;
        self.decode_string_content(c)
    }

    pub fn decode_string_content(&mut self, c: codes::Code) -> Result<String, RMError> {
        let n = self.bytes_len(c)?;
        if n <= 0 {
            return Ok("".to_string());
        }
        let b = self.read_n(n)?;
        let s = String::from_utf8(b)?;
        Ok(s)
    }

    pub fn decode_bytes(&mut self) -> Result<Vec<u8>, RMError> {
        let c = self.read_code()?;
        self.decode_bytes_content(c)
    }

    pub fn decode_bytes_content(&mut self, c: codes::Code) -> Result<Vec<u8>, RMError> {
        let n = self.bytes_len(c)?;
        if n == -1 {
            return Ok(Vec::new());
        }
        let b = self.read_n(n)?;
        Ok(b)
    }
}

impl<'a> Decoder<'a> {
    pub fn decode_bool(&mut self) -> Result<bool, RMError> {
        let c = self.read_code()?;
        self.read_bool(c)
    }

    pub fn read_bool(&mut self, c: codes::Code) -> Result<bool, RMError> {
        match c {
            codes::FALSE => Ok(false),
            codes::TRUE => Ok(true),
            _ => Err(RMError::InvalidCode(c)),
        }
    }
}

impl<'a> Decoder<'a> {
    pub fn decode_float32(&mut self) -> Result<f32, RMError> {
        let c = self.read_code()?;
        self.read_float32(c)
    }

    pub fn read_float32(&mut self, c: codes::Code) -> Result<f32, RMError> {
        if c == codes::FLOAT_32 {
            let n = self.read_uint32()?;
            return Ok(utils::float32frombits(n));
        }

        let n = self.read_int(c)?;
        Ok(n as f32)
    }

    pub fn decode_float64(&mut self) -> Result<f64, RMError> {
        let c = self.read_code()?;
        self.read_float64(c)
    }

    pub fn read_float64(&mut self, c: codes::Code) -> Result<f64, RMError> {
        if c == codes::FLOAT_32 {
            let n = self.read_float32(c)?;
            return Ok(n as f64);
        }
        if c == codes::FLOAT_64 {
            let n = self.read_uint64()?;
            return Ok(utils::float64frombits(n));
        }

        let n = self.read_int(c)?;
        Ok(n as f64)
    }
}

impl<'a> Decoder<'a> {
    pub fn decode_time(&mut self) -> Result<SystemTime, RMError> {
        let c = self.read_code()?;
        let extlen = self.parse_ext_len(c)?;

        // Skip ext id
        self.read_byte()?;

        let b = self.read_n(extlen)?;
        match b.len() {
            4 => {
                let sec = binary::BigEndian::uint32(&b);
                Ok(SystemTime::UNIX_EPOCH + Duration::new(sec as u64, 0))
            }
            8 => {
                let sec = binary::BigEndian::uint64(&b);
                let nanos = (sec >> 32) as u32;
                let secs = sec & 0x00000003ffffffff;
                Ok(SystemTime::UNIX_EPOCH + Duration::new(secs, nanos))
            }
            12 => {
                let nanos = binary::BigEndian::uint32(&b);
                let secs = binary::BigEndian::uint64(&b[4..]);
                Ok(SystemTime::UNIX_EPOCH + Duration::new(secs, nanos))
            }
            _ => Err(RMError::InvalidExtLen(b.len() as i32)),
        }
    }

    fn parse_ext_len(&mut self, c: codes::Code) -> Result<i32, RMError> {
        match c {
            codes::FIX_EXT_1 => Ok(1),
            codes::FIX_EXT_2 => Ok(2),
            codes::FIX_EXT_4 => Ok(4),
            codes::FIX_EXT_8 => Ok(8),
            codes::FIX_EXT_16 => Ok(16),
            codes::EXT_8 => {
                let n = self.read_uint8()?;
                return Ok(n as i32);
            }
            codes::EXT_16 => {
                let n = self.read_uint16()?;
                return Ok(n as i32);
            }
            codes::EXT_32 => {
                let n = self.read_uint32()?;
                return Ok(n as i32);
            }
            _ => Err(RMError::InvalidCode(c)),
        }
    }
}

impl<'a> Decoder<'a> {
    pub fn decode_array_len(&mut self) -> Result<i32, RMError> {
        let c = self.read_code()?;
        self.array_len(c)
    }

    pub fn array_len(&mut self, c: codes::Code) -> Result<i32, RMError> {
        if c == codes::NIL {
            return Ok(-1);
        }
        if codes::is_fixed_array(c) {
            return Ok((c & codes::FIXED_ARRAY_MASK) as i32);
        }
        match c {
            codes::ARRAY_16 => {
                let n = self.read_uint16()?;
                return Ok(n as i32);
            }
            codes::ARRAY_32 => {
                let n = self.read_uint32()?;
                return Ok(n as i32);
            }
            _ => Err(RMError::InvalidCode(c)),
        }
    }
}

impl<'a> Decoder<'a> {
    pub fn decode_map_len(&mut self) -> Result<i32, RMError> {
        let c = self.read_code()?;
        self.map_len(c)
    }

    pub fn map_len(&mut self, c: codes::Code) -> Result<i32, RMError> {
        if c == codes::NIL {
            return Ok(-1);
        }
        if codes::is_fixed_map(c) {
            return Ok((c & codes::FIXED_MAP_MASK) as i32);
        }
        match c {
            codes::MAP_16 => {
                let n = self.read_uint16()?;
                return Ok(n as i32);
            }
            codes::MAP_32 => {
                let n = self.read_uint32()?;
                return Ok(n as i32);
            }
            _ => Err(RMError::InvalidCode(c)),
        }
    }
}

pub fn decode_to_value(v: &[u8]) -> Result<Value, RMError> {
    let mut dec = Decoder::new(&v);
    decode_to_value_inner(&mut dec)
}

fn decode_to_value_inner(mut dec: &mut Decoder) -> Result<Value, RMError> {
    let c = dec.read_code()?;
    let result = match c {
        codes::NIL => Value::Null,
        codes::FALSE | codes::TRUE => {
            let result = dec.read_bool(c)?;
            Value::Bool(result)
        }
        codes::FLOAT_32 => {
            let result = dec.read_float32(c)?;
            Value::Number(result.to_string())
        }
        codes::FLOAT_64 => {
            let result = dec.read_float64(c)?;
            Value::Number(result.to_string())
        }
        codes::UINT_8 | codes::UINT_16 | codes::UINT_32 | codes::UINT_64 => {
            let result = dec.read_uint(c)?;
            Value::Number(result.to_string())
        }
        codes::INT_8 | codes::INT_16 | codes::INT_32 | codes::INT_64 => {
            let result = dec.read_int(c)?;
            Value::Number(result.to_string())
        }
        codes::STR_8 | codes::STR_16 | codes::STR_32 => {
            let result = dec.decode_string_content(c)?;
            Value::String(result)
        }
        codes::BIN_8 | codes::BIN_16 | codes::BIN_32 => {
            let bytes_array = dec.decode_bytes_content(c)?;
            let result = unsafe { String::from_utf8_unchecked(bytes_array) };
            Value::String(result)
        }
        _ => {
            if codes::is_fixed_num(c) {
                let result = dec.read_int(c)?;
                return Ok(Value::Number(result.to_string()));
            }
            if codes::is_fixed_string(c) {
                let result = dec.decode_string_content(c)?;
                return Ok(Value::String(result));
            }
            if codes::is_fixed_array(c) || c == codes::ARRAY_16 || c == codes::ARRAY_32 {
                let arraylen = dec.array_len(c)?;
                let mut result: Vec<Value> = Vec::new();
                let mut i = 0;
                while i < arraylen {
                    let sub: Value = decode_to_value_inner(&mut dec)?;
                    result.push(sub);
                    i += 1;
                }
                return Ok(Value::Array(result));
            }
            if codes::is_fixed_map(c) || c == codes::MAP_16 || c == codes::MAP_32 {
                let maplen = dec.map_len(c)?;
                let mut result: HashMap<String, Value> = HashMap::new();
                let mut i = 0;
                while i < maplen {
                    let mapkey: Value = decode_to_value_inner(&mut dec)?;
                    let mapvalue: Value = decode_to_value_inner(&mut dec)?;
                    result.insert(mapkey.get_string(), mapvalue);
                    i += 1;
                }
                return Ok(Value::Object(result));
            }
            Value::default()
        }
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_bool() {
        let buf = vec![0xc3];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_bool().unwrap();
        assert_eq!(result, true);

        let buf = vec![0xc2];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_bool().unwrap();
        assert_eq!(result, false);

        let buf = vec![0xc2, 0xc3];
        let mut dec = Decoder::new(&buf);
        let result1 = dec.decode_bool().unwrap();
        let result2 = dec.decode_bool().unwrap();
        assert_eq!(result1, false);
        assert_eq!(result2, true);

        let buf = vec![0xc3, 0xc2];
        let mut dec = Decoder::new(&buf);
        let result1 = dec.decode_bool().unwrap();
        let result2 = dec.decode_bool().unwrap();
        assert_eq!(result1, true);
        assert_eq!(result2, false);
    }

    #[test]
    fn test_decode_empty_string() {
        let buf = vec![0xa0];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_string().unwrap();
        assert_eq!(result, "".to_string());
    }

    #[test]
    fn test_decode_fix_string() {
        let buf = vec![0xA5, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_string().unwrap();
        assert_eq!(result, "hello".to_string());
    }

    #[test]
    fn test_decode_string_1() {
        let buf = vec![
            0xD9, 0x20, 0x52, 0x59, 0x55, 0x47, 0x4A, 0x47, 0x57, 0x53, 0x4C, 0x4F, 0x48, 0x41,
            0x4F, 0x50, 0x48, 0x4D, 0x51, 0x4E, 0x57, 0x56, 0x42, 0x57, 0x49, 0x59, 0x45, 0x56,
            0x4F, 0x58, 0x44, 0x54, 0x4B, 0x55,
        ];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_string().unwrap();
        assert_eq!(result, "RYUGJGWSLOHAOPHMQNWVBWIYEVOXDTKU".to_string());
    }

    #[test]
    fn test_decode_bytes_1() {
        let buf = vec![0xC4, 0x02, 0x61, 0x62];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_bytes().unwrap();
        let expectoutput = vec![b'a', b'b'];
        assert_eq!(result, expectoutput);
    }

    #[test]
    fn test_decode_integer() {
        let buf = vec![0xD3, 0xFF, 0xFF, 0xFF, 0xCE, 0x00, 0x00, 0x00, 0x64];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = -214748364700;
        assert_eq!(result, expectoutput);

        let buf = vec![0xD2, 0xFF, 0xFE, 0xEE, 0x90];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = -70000;
        assert_eq!(result, expectoutput);

        let buf = vec![0xD1, 0xFE, 0xD4];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = -300;
        assert_eq!(result, expectoutput);

        let buf = vec![0xD1, 0xFF, 0x38];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = -200;
        assert_eq!(result, expectoutput);

        let buf = vec![0xD0, 0x9C];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = -100;
        assert_eq!(result, expectoutput);

        let buf = vec![0xFF];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = -1;
        assert_eq!(result, expectoutput);

        let buf = vec![0x00];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = 0;
        assert_eq!(result, expectoutput);

        let buf = vec![0x01];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = 1;
        assert_eq!(result, expectoutput);

        let buf = vec![0x64];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = 100;
        assert_eq!(result, expectoutput);

        let buf = vec![0xcc, 0xc8];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = 200;
        assert_eq!(result, expectoutput);

        let buf = vec![0xcd, 0x01, 0x2c];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = 300;
        assert_eq!(result, expectoutput);

        let buf = vec![0xCE, 0x00, 0x01, 0x11, 0x70];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = 70000;
        assert_eq!(result, expectoutput);

        let buf = vec![0xCF, 0x00, 0x00, 0x00, 0x31, 0xFF, 0xFF, 0xFF, 0x9C];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_int64().unwrap();
        let expectoutput = 214748364700;
        assert_eq!(result, expectoutput);
    }

    #[test]
    fn test_decode_float() {
        let buf = vec![0xCA, 0x3F, 0x9E, 0x06, 0x10];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_float32().unwrap();
        let expectoutput = 1.23456;
        assert_eq!(result, expectoutput);

        let buf = vec![0xCB, 0x3F, 0xF3, 0xC0, 0xC1, 0xFC, 0x8F, 0x32, 0x38];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_float64().unwrap();
        let expectoutput = 1.23456;
        assert_eq!(result, expectoutput);

        let buf = vec![0xCA, 0x3E, 0xAA, 0xAA, 0xAB];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_float32().unwrap();
        let expectoutput = 1.0 / 3.0;
        assert_eq!(result, expectoutput);

        let buf = vec![0xCB, 0x3F, 0xD5, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
        let mut dec = Decoder::new(&buf);
        let result = dec.decode_float64().unwrap();
        let expectoutput = 1.0 / 3.0;
        assert_eq!(result, expectoutput);
    }
}
