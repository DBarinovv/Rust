extern crate proc_macro_impl;
// use proc_macro_impl::make_answer;
// use proc_macro_simpl::AnswerFn;
// use proc_macro_impl::HelperAttr;
use proc_macro_impl::show_streams;

// make_answer!();

// #[derive(AnswerFn)]
// struct Test;

// #[derive(HelperAttr)]
// struct add_field {
//     #[helper] 
//     field: ()
// }

// Example: Basic function
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

// Example: Attribute with input
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

// Example: Multiple tokens in the input
#[show_streams(multiple => tokens)]
fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

// Example:
#[show_streams { delimiters }]
fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"

fn main() {
    // println!("{}", answer());
    invoke1();
    invoke2();
    invoke3();
    invoke4();
}
 