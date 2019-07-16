use crate::binary;
use std::time::SystemTime;

pub fn encode_time(t: SystemTime) -> Vec<u8> {
    match t.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let secs = n.as_secs();
            if secs >> 34 == 0 {
                let data = ((n.subsec_nanos() as u64) << 34) | secs;
                if data & 0xffffffff00000000 == 0 {
                    let mut b = vec![0; 4];
                    binary::BigEndian::put_uint32(&mut b, data as u32);
                    return b;
                } else {
                    let mut b = vec![0; 8];
                    binary::BigEndian::put_uint64(&mut b, data);
                    return b;
                }
            }

            let mut b = vec![0; 12];
            binary::BigEndian::put_uint32(&mut b, n.subsec_nanos());
            binary::BigEndian::put_uint64(&mut b[4..], secs);
            return b;
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
