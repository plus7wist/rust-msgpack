use crate::value::*;
use std::collections::HashMap;

pub trait IntoValue<T>: Default {
    fn into_value(self) -> Value;
}

impl IntoValue<bool> for bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl IntoValue<String> for String {
    fn into_value(self) -> Value {
        Value::String(self.clone())
    }
}

impl IntoValue<Vec<Value>> for Vec<Value> {
    fn into_value(self) -> Value {
        Value::Array(self)
    }
}

impl IntoValue<HashMap<String, Value>> for HashMap<String, Value> {
    fn into_value(self) -> Value {
        Value::Object(self)
    }
}

impl IntoValue<u8> for u8 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<i8> for i8 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<u16> for u16 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<i16> for i16 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<u32> for u32 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<i32> for i32 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<u64> for u64 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<i64> for i64 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<f32> for f32 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<f64> for f64 {
    fn into_value(self) -> Value {
        Value::Number(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_into_value() {
        let v1: Value = Value::Null;
        let mut arr: Vec<Value> = Vec::new();
        arr.push(v1);
        println!("arr = {:?}", arr);
        let result = arr.into_value();
        // println!("arr = {:?}", arr);
    }
}
