pub struct LittleEndian {}

impl LittleEndian {
    pub fn uint16(b: &[u8]) -> u16 {
        (b[0] as u16) | ((b[1] as u16) << 8)
    }

    pub fn uint32(b: &[u8]) -> u32 {
        (b[0] as u32) | ((b[1] as u32) << 8) | ((b[2] as u32) << 16) | ((b[3] as u32) << 24)
    }

    pub fn uint64(b: &[u8]) -> u64 {
        (b[0] as u64)
            | ((b[1] as u64) << 8)
            | ((b[2] as u64) << 16)
            | ((b[3] as u64) << 24)
            | ((b[4] as u64) << 32)
            | ((b[5] as u64) << 40)
            | ((b[6] as u64) << 48)
            | ((b[7] as u64) << 56)
    }

    pub fn put_uint16(b: &mut [u8], v: u16) {
        b[0] = v as u8;
        b[1] = (v >> 8) as u8;
    }

    pub fn put_uint32(b: &mut [u8], v: u32) {
        b[0] = v as u8;
        b[1] = (v >> 8) as u8;
        b[2] = (v >> 16) as u8;
        b[3] = (v >> 24) as u8;
    }

    pub fn put_uint64(b: &mut [u8], v: u64) {
        b[0] = v as u8;
        b[1] = (v >> 8) as u8;
        b[2] = (v >> 16) as u8;
        b[3] = (v >> 24) as u8;
        b[4] = (v >> 32) as u8;
        b[5] = (v >> 40) as u8;
        b[6] = (v >> 48) as u8;
        b[7] = (v >> 56) as u8;
    }

    pub fn string() -> String {
        "LittleEndian".to_string()
    }
}

pub struct BigEndian {}

impl BigEndian {
    pub fn uint16(b: &[u8]) -> u16 {
        (b[1] as u16) | ((b[0] as u16) << 8)
    }

    pub fn uint32(b: &[u8]) -> u32 {
        (b[3] as u32) | ((b[2] as u32) << 8) | ((b[1] as u32) << 16) | ((b[0] as u32) << 24)
    }

    pub fn uint64(b: &[u8]) -> u64 {
        (b[7] as u64)
            | ((b[6] as u64) << 8)
            | ((b[5] as u64) << 16)
            | ((b[4] as u64) << 24)
            | ((b[3] as u64) << 32)
            | ((b[2] as u64) << 40)
            | ((b[1] as u64) << 48)
            | ((b[0] as u64) << 56)
    }

    pub fn put_uint16(b: &mut [u8], v: u16) {
        b[1] = v as u8;
        b[0] = (v >> 8) as u8;
    }

    pub fn put_uint32(b: &mut [u8], v: u32) {
        b[3] = v as u8;
        b[2] = (v >> 8) as u8;
        b[1] = (v >> 16) as u8;
        b[0] = (v >> 24) as u8;
    }

    pub fn put_uint64(b: &mut [u8], v: u64) {
        b[7] = v as u8;
        b[6] = (v >> 8) as u8;
        b[5] = (v >> 16) as u8;
        b[4] = (v >> 24) as u8;
        b[3] = (v >> 32) as u8;
        b[2] = (v >> 40) as u8;
        b[1] = (v >> 48) as u8;
        b[0] = (v >> 56) as u8;
    }

    pub fn string() -> String {
        "BigEndian".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_little_endian_u16() {
        let mut b = vec![0; 2];
        let v = 256;
        LittleEndian::put_uint16(&mut b, v);
        let v2 = LittleEndian::uint16(&b);
        assert_eq!(v, v2);
    }

    #[test]
    fn test_little_endian_u32() {
        let mut b = vec![0; 4];
        let v = 25611111;
        LittleEndian::put_uint32(&mut b, v);
        let v2 = LittleEndian::uint32(&b);
        assert_eq!(v, v2);
    }

    #[test]
    fn test_little_endian_u64() {
        let mut b = vec![0; 8];
        let v = 2561111883472521;
        LittleEndian::put_uint64(&mut b, v);
        let v2 = LittleEndian::uint64(&b);
        assert_eq!(v, v2);
    }

    #[test]
    fn test_big_endian_u16() {
        let mut b = vec![0; 2];
        let v = 256;
        BigEndian::put_uint16(&mut b, v);
        let v2 = BigEndian::uint16(&b);
        assert_eq!(v, v2);
    }

    #[test]
    fn test_big_endian_u32() {
        let mut b = vec![0; 4];
        let v = 25611111;
        BigEndian::put_uint32(&mut b, v);
        let v2 = BigEndian::uint32(&b);
        assert_eq!(v, v2);
    }

    #[test]
    fn test_big_endian_u64() {
        let mut b = vec![0; 8];
        let v = 2561111883472521;
        BigEndian::put_uint64(&mut b, v);
        let v2 = BigEndian::uint64(&b);
        assert_eq!(v, v2);
    }
}
