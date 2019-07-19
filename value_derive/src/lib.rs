#![recursion_limit = "1024"]
extern crate proc_macro;

mod from_value_derive;
mod into_value_derive;

#[proc_macro_derive(FromValue)]
pub fn from_value_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_value_derive::from_value_derive_inner(input)
}

#[proc_macro_derive(IntoValue)]
pub fn into_value_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    into_value_derive::into_value_derive_inner(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
