use crate::codes;
use crate::decode::Decoder;
use crate::encode::Encoder;
use std::collections::HashMap;
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
    let mut dec = Decoder::new(&v);
    msgpack_into_value_inner(&mut dec)
}

pub fn msgpack_into_value_inner(mut dec: &mut Decoder) -> Value {
    let c = dec.read_code().unwrap();
    let result = match c {
        codes::NIL => Value::Null,
        codes::FALSE | codes::TRUE => {
            let result = dec.read_bool(c).unwrap();
            Value::Bool(result)
        }
        codes::FLOAT_32 => {
            let result = dec.read_float32(c).unwrap();
            Value::Number(result.to_string())
        }
        codes::FLOAT_64 => {
            let result = dec.read_float64(c).unwrap();
            Value::Number(result.to_string())
        }
        codes::UINT_8 | codes::UINT_16 | codes::UINT_32 | codes::UINT_64 => {
            let result = dec.read_uint(c).unwrap();
            Value::Number(result.to_string())
        }
        codes::INT_8 | codes::INT_16 | codes::INT_32 | codes::INT_64 => {
            let result = dec.read_int(c).unwrap();
            Value::Number(result.to_string())
        }
        codes::STR_8 | codes::STR_16 | codes::STR_32 => {
            let result = dec.decode_string_content(c).unwrap();
            Value::String(result)
        }
        codes::BIN_8 | codes::BIN_16 | codes::BIN_32 => {
            let bytes_array = dec.decode_bytes_content(c).unwrap();
            let result = unsafe { String::from_utf8_unchecked(bytes_array) };
            Value::String(result)
        }
        _ => {
            if codes::is_fixed_num(c) {
                let result = dec.read_int(c).unwrap();
                return Value::Number(result.to_string());
            }
            if codes::is_fixed_string(c) {
                let result = dec.decode_string_content(c).unwrap();
                return Value::String(result);
            }
            if codes::is_fixed_array(c) || c == codes::ARRAY_16 || c == codes::ARRAY_32 {
                let arraylen = dec.array_len(c).unwrap();
                let mut result: Vec<Value> = Vec::new();
                let mut i = 0;
                while i < arraylen {
                    let sub: Value = msgpack_into_value_inner(&mut dec);
                    result.push(sub);
                    i += 1;
                }
                return Value::Array(result);
            }
            if codes::is_fixed_map(c) || c == codes::MAP_16 || c == codes::MAP_32 {
                let maplen = dec.map_len(c).unwrap();
                let mut result: HashMap<String, Value> = HashMap::new();
                let mut i = 0;
                while i < maplen {
                    let mapkey: Value = msgpack_into_value_inner(&mut dec);
                    let mapvalue: Value = msgpack_into_value_inner(&mut dec);
                    result.insert(mapkey.get_string(), mapvalue);
                    i += 1;
                }
                return Value::Object(result);
            }
            Value::default()
        }
    };
    result
}
