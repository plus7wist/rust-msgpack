use crate::bytes;

pub struct Decoder<'a> {
    r: bytes::Reader<'a>,
}

impl<'a> Decoder<'a> {
    pub fn new(s: &'a [u8]) -> Decoder {
        Decoder {
            r: bytes::Reader::new(s),
        }
    }

    fn read_code(&mut self) {}

    pub fn decode_string(&mut self) -> Result<String, &str> {
        Ok(())
    }
}
