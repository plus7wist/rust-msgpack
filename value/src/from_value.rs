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

impl<T> FromValue<Vec<T>> for Value
where
    Value: FromValue<T>,
{
    fn from_value(&self) -> Vec<T> {
        match self {
            Value::Array(array) => array.iter().map(|x| x.from_value()).collect(),
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

macro_rules! mark_trait {
    ($trait: ident; $($type: ty), *) => {
        $(impl $trait for $type {})*
    }
}

pub trait ToNumber {}
mark_trait!{ToNumber; i8, u8, i16, u16, i32, u32, i64, u64, f32, f64}

impl<T> FromValue<T> for Value
where
    T: std::str::FromStr + ToNumber,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn from_value(&self) -> T {
        match self {
            Value::Number(n) => n.parse().unwrap(),
            _ => panic!("invalid value"),
        }
    }
}
