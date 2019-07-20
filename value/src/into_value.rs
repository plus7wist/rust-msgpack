use crate::value::*;
use std::collections::HashMap;

pub trait IntoValue<T>: Default {
    fn into_value(&self) -> Value;
}

impl IntoValue<Value> for Value {
    fn into_value(&self) -> Value {
        self.clone()
    }
}

impl IntoValue<bool> for bool {
    fn into_value(&self) -> Value {
        Value::Bool(*self)
    }
}

impl IntoValue<String> for String {
    fn into_value(&self) -> Value {
        Value::String(self.clone())
    }
}

impl<T> IntoValue<Vec<T>> for Vec<T>
where
    T: IntoValue<T> + Clone,
{
    fn into_value(&self) -> Value {
        let mut result: Vec<Value> = Vec::new();
        for x in self {
            result.push(x.clone().into_value());
        }
        Value::Array(result)
    }
}

impl<HK, HV> IntoValue<HashMap<HK, HV>> for HashMap<HK, HV>
where
    HK: std::str::FromStr + std::string::ToString + std::hash::Hash + std::cmp::Eq,
    HV: IntoValue<HV>,
{
    fn into_value(&self) -> Value {
        let mut result: HashMap<String, Value> = HashMap::new();
        let keys: Vec<String> = self.keys().map(|k| k.to_string()).collect();
        for key in keys {
            let k = HK::from_str(&key);
            match k {
                Ok(k) => {
                    let value = self.get(&k).unwrap();
                    result.insert(key.clone().to_string(), value.into_value());
                }
                Err(_) => panic!("invalid IntoValue<HashMap<HK, HV>>"),
            }
        }
        Value::Object(result)
    }
}

impl IntoValue<u8> for u8 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<i8> for i8 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<u16> for u16 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<i16> for i16 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<u32> for u32 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<i32> for i32 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<u64> for u64 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<i64> for i64 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<f32> for f32 {
    fn into_value(&self) -> Value {
        Value::Number(self.to_string())
    }
}

impl IntoValue<f64> for f64 {
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
