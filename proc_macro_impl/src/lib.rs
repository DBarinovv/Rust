extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn make_function(item: TokenStream) -> TokenStream {
    // println!("\n\nITEM: {:?} \n\n", item.to_string());
    // let input = parse_macro_input!(item as DeriveInput);
    // let ast = syn::parse(item).unwrap();
    // println!("Input = {:?}", input.ident);
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
