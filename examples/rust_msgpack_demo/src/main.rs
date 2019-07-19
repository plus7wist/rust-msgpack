use rust_msgpack::transvalue;
use std::collections::HashMap;
use value::from_value::FromValue;
use value::into_value::IntoValue;
use value::value::Value;
use value_derive::*;

#[derive(Debug, Default, FromValue, IntoValue)]
struct Student {
    name: String,
    age: i32,
    sub: Sub,
}

#[derive(Debug, Default, FromValue, IntoValue)]
struct Sub {
    a: i32,
    b: bool,
    c: HashMap<String, String>,
}

fn main() {
    let mut s1 = Student {
        name: "huangjian".to_string(),
        age: 10000,
        sub: Sub {
            a: 100,
            ..Default::default()
        },
        ..Default::default()
    };
    s1.sub.c = HashMap::new();
    s1.sub.c.insert("language".to_string(), "Rust".to_string());

    println!("s1 = {:?}\n", s1);

    let v: Value = s1.into_value();
    println!("v = {}", v);

    let bin = transvalue::msgpack_from_value(&v);
    println!("bin = {:?}", bin);

    //let s2: Student = v.from_value();
    //println!("s2 = {:?}", s2);
}
