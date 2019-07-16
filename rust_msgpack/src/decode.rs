use crate::binary;
use crate::bytes;
use crate::codes;
use crate::error::Error as RMError;

pub struct Decoder<'a> {
    r: bytes::Reader<'a>,
}

impl<'a> Decoder<'a> {
    pub fn new(s: &'a [u8]) -> Decoder {
        Decoder {
            r: bytes::Reader::new(s),
        }
    }

    fn read_code(&mut self) -> Result<codes::Code, RMError> {
        let c = self.r.read_byte()?;
        Ok(c as codes::Code)
    }

    fn read_n(&mut self, n: i32) -> Result<Vec<u8>, RMError> {
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

    fn read_int(&mut self, c: codes::Code) -> Result<i64, RMError> {
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
            codes::UINT_64 | codes::INT_64 => {
                let n = self.read_uint64()?;
                return Ok(n as i64);
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
}

impl<'a> Decoder<'a> {
    pub fn decode_string(&mut self) -> Result<String, RMError> {
        let c = self.read_code()?;
        self.decode_string_content(c)
    }

    fn decode_string_content(&mut self, c: codes::Code) -> Result<String, RMError> {
        let n = self.bytes_len(c)?;
        if n == -1 {
            return Ok("".to_string());
        }
        let b = self.read_n(n)?;
        let s = String::from_utf8(b)?;
        Ok(s)
    }
}
