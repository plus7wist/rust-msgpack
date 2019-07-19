use crate::value::*;
use std::collections::HashMap;

pub trait FromValue<T>: Default {
    fn from_value(self) -> T;
}

impl FromValue<bool> for Value {
    fn from_value(self) -> bool {
        match self {
            Value::Bool(n) => n,
            _ => panic!("invalid value for bool"),
        }
    }
}

impl FromValue<String> for Value {
    fn from_value(self) -> String {
        match self {
            Value::String(s) => s,
            _ => panic!("invalid value for String"),
        }
    }
}

impl FromValue<Vec<Value>> for Value {
    fn from_value(self) -> Vec<Value> {
        match self {
            Value::Array(a) => a,
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<HashMap<String, Value>> for Value {
    fn from_value(self) -> HashMap<String, Value> {
        match self {
            Value::Object(hm) => hm,
            _ => panic!("invalid value for bool"),
        }
    }
}

impl FromValue<u8> for Value {
    fn from_value(self) -> u8 {
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
    fn from_value(self) -> i8 {
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
    fn from_value(self) -> u16 {
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
    fn from_value(self) -> i16 {
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
    fn from_value(self) -> u32 {
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
    fn from_value(self) -> i32 {
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
    fn from_value(self) -> u64 {
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
    fn from_value(self) -> i64 {
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
    fn from_value(self) -> f32 {
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
    fn from_value(self) -> f64 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<f64>().unwrap();
                num
            }
            _ => panic!("invalid value for f64"),
        }
    }
}
