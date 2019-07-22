use crate::value::*;
use std::collections::HashMap;

pub trait FromValue<T>: Default {
    fn from_value(&self) -> T;
}

impl FromValue<Value> for Value {
    fn from_value(&self) -> Value {
        self.clone()
    }
}

impl FromValue<bool> for Value {
    fn from_value(&self) -> bool {
        match self {
            Value::Bool(n) => *n,
            _ => panic!("invalid value for bool"),
        }
    }
}

impl FromValue<String> for Value {
    fn from_value(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            _ => panic!("invalid value for String"),
        }
    }
}

impl FromValue<Vec<Value>> for Value {
    fn from_value(&self) -> Vec<Value> {
        match self {
            Value::Array(a) => a.clone(),
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<Vec<u8>> for Value {
    fn from_value(&self) -> Vec<u8> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<u8> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            Value::String(s) => {
                let mut result: Vec<u8> = Vec::new();
                for x in s.as_bytes() {
                    result.push(x.clone());
                }
                result
            }
            _ => panic!("invalid value for Array, type = {}", self.get_type()),
        }
    }
}

impl FromValue<Vec<i8>> for Value {
    fn from_value(&self) -> Vec<i8> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<i8> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<Vec<u16>> for Value {
    fn from_value(&self) -> Vec<u16> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<u16> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<Vec<i16>> for Value {
    fn from_value(&self) -> Vec<i16> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<i16> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<Vec<u32>> for Value {
    fn from_value(&self) -> Vec<u32> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<u32> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<Vec<i32>> for Value {
    fn from_value(&self) -> Vec<i32> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<i32> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<Vec<u64>> for Value {
    fn from_value(&self) -> Vec<u64> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<u64> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<Vec<i64>> for Value {
    fn from_value(&self) -> Vec<i64> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<i64> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<Vec<f32>> for Value {
    fn from_value(&self) -> Vec<f32> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<f32> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<Vec<f64>> for Value {
    fn from_value(&self) -> Vec<f64> {
        match self {
            Value::Array(a) => {
                let mut result: Vec<f64> = Vec::new();
                for x in a {
                    result.push(x.clone().from_value());
                }
                result
            }
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<HashMap<String, Value>> for Value {
    fn from_value(&self) -> HashMap<String, Value> {
        match self {
            Value::Object(hm) => hm.clone(),
            _ => panic!("invalid value for HashMap"),
        }
    }
}

impl FromValue<HashMap<String, String>> for Value {
    fn from_value(&self) -> HashMap<String, String> {
        match self {
            Value::Object(hm) => {
                let mut result: HashMap<String, String> = HashMap::new();
                for key in hm.keys() {
                    let value = hm.get(key).unwrap();
                    result.insert(key.clone(), value.from_value());
                }
                result
            }
            _ => panic!("invalid value for HashMap"),
        }
    }
}

impl FromValue<u8> for Value {
    fn from_value(&self) -> u8 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<u8>().unwrap();
                num
            }
            _ => panic!("invalid value for u8"),
        }
    }
}

impl FromValue<i8> for Value {
    fn from_value(&self) -> i8 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<i8>().unwrap();
                num
            }
            _ => panic!("invalid value for i8"),
        }
    }
}

impl FromValue<u16> for Value {
    fn from_value(&self) -> u16 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<u16>().unwrap();
                num
            }
            _ => panic!("invalid value for u16"),
        }
    }
}

impl FromValue<i16> for Value {
    fn from_value(&self) -> i16 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<i16>().unwrap();
                num
            }
            _ => panic!("invalid value for i16"),
        }
    }
}

impl FromValue<u32> for Value {
    fn from_value(&self) -> u32 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<u32>().unwrap();
                num
            }
            _ => panic!("invalid value for u32"),
        }
    }
}

impl FromValue<i32> for Value {
    fn from_value(&self) -> i32 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<i32>().unwrap();
                num
            }
            _ => panic!("invalid value for i32"),
        }
    }
}

impl FromValue<u64> for Value {
    fn from_value(&self) -> u64 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<u64>().unwrap();
                num
            }
            _ => panic!("invalid value for u64"),
        }
    }
}

impl FromValue<i64> for Value {
    fn from_value(&self) -> i64 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<i64>().unwrap();
                num
            }
            _ => panic!("invalid value for i64"),
        }
    }
}

impl FromValue<f32> for Value {
    fn from_value(&self) -> f32 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<f32>().unwrap();
                num
            }
            _ => panic!("invalid value for f32"),
        }
    }
}

impl FromValue<f64> for Value {
    fn from_value(&self) -> f64 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<f64>().unwrap();
                num
            }
            _ => panic!("invalid value for f64"),
        }
    }
}
