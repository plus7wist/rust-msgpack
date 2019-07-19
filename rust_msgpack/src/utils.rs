use std::cmp::min;

pub fn float32bits(f: f32) -> u32 {
    unsafe { std::mem::transmute(f) }
}

pub fn float32frombits(b: u32) -> f32 {
    unsafe { std::mem::transmute(b) }
}

pub fn float64bits(f: f64) -> u64 {
    unsafe { std::mem::transmute(f) }
}

pub fn float64frombits(b: u64) -> f64 {
    unsafe { std::mem::transmute(b) }
}

pub fn slice_copy<T>(dst: &mut [T], src: &[T]) -> i64
where
    T: Copy,
{
    let minlen = min(dst.len(), src.len());
    let src_end_idx = min(minlen, src.len());
    let dst_end_idx = min(minlen, dst.len());
    dst[..dst_end_idx].copy_from_slice(&src[..src_end_idx]);
    minlen as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_bits() {
        assert_eq!(float32bits(1.0f32 / 3.0f32), 1051372203);
        assert_eq!(float32frombits(1051372203), 1.0f32 / 3.0f32);
        assert_eq!(float64bits(1.0f64 / 3.0f64), 4599676419421066581);
        assert_eq!(float64frombits(4599676419421066581), 1.0f64 / 3.0f64);
    }

    #[test]
    fn test_slice_copy() {
        let src = vec![1, 2, 3, 4, 5];
        let mut dst = vec![0; 2];
        slice_copy(&mut dst, &src);
        assert_eq!(&dst, &[1, 2]);
        assert_eq!(&src, &[1, 2, 3, 4, 5]);

        let src = vec![100, 99];
        let mut dst = vec![0; 4];
        assert_eq!(&dst, &[0, 0, 0, 0]);
        slice_copy(&mut dst, &src);
        assert_eq!(&dst, &[100, 99, 0, 0]);
        assert_eq!(&src, &[100, 99]);
    }
}
