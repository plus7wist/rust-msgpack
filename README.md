# rust-msgpack

[msgpack specification](https://github.com/msgpack/msgpack)

## TODO

1. add document
2. add unit test
3. rust_msgpack/src/transvalue.rs, add `fn msgpack_from_multi_values(v: &[&Value]) -> Result<Vec<u8>, RMError>`
4. value/src/from_value.rs, modify trait function to `fn from_value(&self) -> T;`
5. value/src/into_value.rs, modify trait function to `fn into_value(&self) -> Value;`
6. remove no used code.

