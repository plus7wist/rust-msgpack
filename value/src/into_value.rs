use crate::value::*;
use std::collections::HashMap;

pub trait IntoValue: Default {
    fn into_value(&self) -> Value;
}

impl IntoValue for Value {
    fn into_value(&self) -> Value {
        self.clone()
    }
}

impl IntoValue for bool {
    fn into_value(&self) -> Value {
        Value::Bool(*self)
    }
}

impl IntoValue for String {
    fn into_value(&self) -> Value {
        Value::String(self.clone())
    }
}

impl IntoValue for &str {
    fn into_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

impl<T> IntoValue for Vec<T>
where
    T: IntoValue + Clone,
{
    fn into_value(&self) -> Value {
        let mut result: Vec<Value> = Vec::new();
        for x in self {
            result.push(x.clone().into_value());
        }
        Value::Array(result)
    }
}

impl<HK, HV> IntoValue for HashMap<HK, HV>
where
    HK: std::string::ToString + std::hash::Hash + std::cmp::Eq,
    HV: IntoValue,
{
    fn into_value(&self) -> Value {
        let mut result: HashMap<String, Value> = HashMap::new();
        for key in self.keys() {
            let value = self.get(key).unwrap();
            result.insert(key.to_string(), value.into_value());
        }
        Value::Object(result)
    }
}

impl IntoValue for u8 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue for i8 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue for u16 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue for i16 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue for u32 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue for i32 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue for u64 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue for i64 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue for f32 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue for f64 {
    fn into_value(&self) -> Value {
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
