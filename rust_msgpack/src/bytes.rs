use crate::utils;

pub struct Reader<'a> {
    s: &'a [u8],
    i: i64,
}

impl<'a> Reader<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        Self { s: s, i: 0 }
    }

    // len returns the number of bytes of the unread portion of the slice.
    pub fn len(&self) -> i64 {
        if self.i >= (self.s.len() as i64) {
            return 0;
        }
        (self.s.len() as i64) - self.i
    }

    pub fn size(&self) -> i64 {
        self.s.len() as i64
    }

    pub fn read(&mut self, mut b: &mut [u8]) -> i64 {
        if self.i >= (self.s.len() as i64) {
            return 0;
        }

        let start_idx = self.i as usize;
        let n = utils::slice_copy(&mut b, &self.s[start_idx..]);
        if n == 0 {
            return 0;
        }

        self.i += n;
        n
    }

    pub fn read_at(&mut self, mut b: &mut [u8], off: i64) -> i64 {
        if off < 0 {
            return 0;
        }

        if off >= (self.s.len() as i64) {
            return 0;
        }

        let start_idx = off as usize;
        utils::slice_copy(&mut b, &self.s[start_idx..])
    }

    pub fn read_byte(&mut self) -> Result<u8, &str> {
        if self.i >= (self.s.len() as i64) {
            return Err("End Of File");
        }

        let b = self.s[self.i as usize];
        self.i += 1;
        Ok(b)
    }

    pub fn reset(&mut self) {
        self.i = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_reader() {
        let a = String::from("hello");
        let b = a.as_bytes();
        let mut r = Reader::new(&b);
        assert_eq!(r.len(), 5);
        assert_eq!(r.size(), 5);

        let one = r.read_byte();
        assert_eq!(one, Ok(b'h'));
        assert_eq!(r.len(), 4);
        assert_eq!(r.size(), 5);

        let one = r.read_byte();
        assert_eq!(one, Ok(b'e'));
        assert_eq!(r.len(), 3);
        assert_eq!(r.size(), 5);

        let one = r.read_byte();
        assert_eq!(one, Ok(b'l'));
        assert_eq!(r.len(), 2);
        assert_eq!(r.size(), 5);

        let one = r.read_byte();
        assert_eq!(one, Ok(b'l'));
        assert_eq!(r.len(), 1);
        assert_eq!(r.size(), 5);

        let one = r.read_byte();
        assert_eq!(one, Ok(b'o'));
        assert_eq!(r.len(), 0);
        assert_eq!(r.size(), 5);

        let one = r.read_byte();
        assert_eq!(one, Err("End Of File"));

        r.reset();
        let mut dst = vec![b'0'; 7];
        r.read(&mut dst);
        assert_eq!(&dst, &[b'h', b'e', b'l', b'l', b'o', b'0', b'0']);

        r.reset();
        let mut dst = vec![b'0'; 2];
        r.read(&mut dst);
        assert_eq!(&dst, &[b'h', b'e']);

        r.reset();
        let mut dst = vec![b'0'; 2];
        r.read_at(&mut dst, 0);
        assert_eq!(&dst, &[b'h', b'e']);

        r.reset();
        let mut dst = vec![b'0'; 2];
        r.read_at(&mut dst, 1);
        assert_eq!(&dst, &[b'e', b'l']);

        r.reset();
        let mut dst = vec![b'0'; 2];
        r.read_at(&mut dst, 2);
        assert_eq!(&dst, &[b'l', b'l']);

        r.reset();
        let mut dst = vec![b'0'; 2];
        r.read_at(&mut dst, 3);
        assert_eq!(&dst, &[b'l', b'o']);

        r.reset();
        let mut dst = vec![b'0'; 2];
        r.read_at(&mut dst, 4);
        assert_eq!(&dst, &[b'o', b'0']);
    }
}
