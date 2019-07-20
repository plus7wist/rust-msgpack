use rust_msgpack::decode;
use rust_msgpack::encode;
use std::collections::HashMap;
use value::from_value::FromValue;
use value::into_value::IntoValue;
use value_derive::*;

#[derive(Debug, Default, FromValue, IntoValue)]
struct Student {
    name: String,
    age: i32,
}

fn main() {
    let mut hm: HashMap<String, String> = HashMap::new();
    hm.insert("type".to_string(), "student".to_string());
    hm.insert("name".to_string(), "huangjian".to_string());

    println!("hm = {:?}\n", hm);

    let bin = encode::encode(&hm).unwrap();
    println!("bin = {:?}\n", bin);

    let v2 = decode::decode_to_value(&bin).unwrap();
    println!("v2 = {}", v2);

    let s2: Student = v2.from_value();
    println!("s2 = {:?}", s2);
}
