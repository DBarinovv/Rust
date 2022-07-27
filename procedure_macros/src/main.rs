extern crate proc_macro_impl;
use proc_macro_impl::make_answer;

make_answer!();

fn main() {
    println!("{}", answer());
}
