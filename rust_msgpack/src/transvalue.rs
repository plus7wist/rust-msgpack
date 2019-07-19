use crate::encode::Encoder;
use value::value::Value;

pub fn msgpack_from_value(v: &Value) -> Vec<u8> {
    let mut enc = Encoder::new();
    match v {
        Value::Null => enc.encode_nil().unwrap(),
        Value::Bool(b) => enc.encode_bool(*b).unwrap(),
        Value::Number(n) => {
            let num = n.parse::<f64>().unwrap();
            enc.encode_float64(num).unwrap()
        }
        Value::String(s) => enc.encode_string(&s).unwrap(),
        Value::Array(arr) => {
            enc.encode_array_len(arr.len() as i32).unwrap();
            for v in arr {
                let sub = msgpack_from_value(v);
                enc.buf.extend(sub);
            }
            ()
        }
        Value::Object(hm) => {
            enc.encode_map_len(hm.len() as i32).unwrap();
            for (key, value) in hm {
                enc.encode_string(key).unwrap();
                let subvalue = msgpack_from_value(value);
                enc.buf.extend(subvalue);
            }
        }
    }
    enc.buf
}

pub fn msgpack_into_value(v: &[u8]) -> Value {
    Value::default()
}
