extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_params(item: TokenStream) -> TokenStream {
    let mut res_func = "fn answer(".to_owned();
    let end_func = ") -> u32 { 42 }";

    let mut arguments = item.to_string();
    arguments = arguments[1..arguments.len() - 1].to_string();

    res_func.push_str(&arguments);
    res_func.push_str(end_func);
    res_func.parse().unwrap()
}
