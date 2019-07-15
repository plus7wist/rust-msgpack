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
}
