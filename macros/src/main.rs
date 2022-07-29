#![allow(dead_code, unused)]

extern crate proc_macro_impl;
use proc_macro_impl::make_params;

macro_rules! create_function {
    ($name:ident, $($args:ident), *) => {
        // let res = self.send(
        //     user,
        //     RMRKAction::RemoveChild {
        //         parent_token_id: parent_token_id.into(),
        //         child_contract_id: child_contract_id.into(),
        //         child_token_id: child_token_id.into(),
        //     },
        // );
        // $name {
        //     $($args: $args.into(), )*
        // }
        // );

        // if let Some(exp_error) = exp_error {
        //     check_run_result_for_error(&res, exp_error);
        // } else {
        //     let reply = RMRKEvent::RemovedChild {
        //         child_contract_id: child_contract_id.into(),
        //         child_token_id: child_token_id.into(),
        //         parent_token_id: parent_token_id.into(),
        //     }
        //     .encode();
        //     assert!(res.contains(&(user, reply)));
        // }
    };
}

#[macro_export]
macro_rules! get_info {
    ($enum_vis:vis enum $enum_name:ident { $( $(patt:meta)? $field_name:ident { $($var_name:ident: $var_type:ty $(,)?)* } ,)* }) => {
        $enum_vis enum $enum_name { 
            $( $field_name { 
                $($var_name: $var_type,)* 
            }, )* 
        }
        impl $enum_name {
            fn get_fields(index: usize) -> String
            {
                const SIZE: usize = $crate::count![@COUNT; $($field_name), *];

                let mut res = Vec::with_capacity(SIZE);
                $(res.push(stringify!($field_name).to_string());)*
                res[index].clone()
            }

            fn get_arguments(index: usize) -> Vec<String> { 
                const SIZE: usize = $crate::count![@COUNT; $($field_name), *];

                let mut res = Vec::with_capacity(SIZE);
                $(res.push(vec![$(stringify!($var_name).to_string()), *]);)*
                res[index].clone()
            }

            fn get_types(index: usize) -> Vec<String> {
                const SIZE: usize = $crate::count![@COUNT; $($field_name), *];

                let mut res = Vec::with_capacity(SIZE);
                $(res.push(vec![$(stringify!($var_type).to_string()), *]);)*
                res[index].clone()
            }

            pub fn get_params(index: usize) -> String {
                let mut res = "".to_owned();
                let args = Self::get_arguments(index);
                let types = Self::get_types(index);

                for ind in 0..args.len() {
                    res.push_str(&args[ind]);
                    res.push_str(&types[ind]);
                }

                res
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:ident), *) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]), *])
    };

    (@SUBST; $_element:ident) => { () };
}

make_params!("x: u128, a: String, b: f32");

get_info! {
enum Actions {
    CreateMe {
        x: u128,
        y: String,
        z: f32
    },
    Pls {
        t: String,
    },
}
}

fn main() {
    let res = answer(0, "AA".to_string(), 1.0);
    println!("res = {:?}", res);

    let index = 0;
    
    let fields = Actions::get_fields(index);
    println!("fields = {:?}", fields);

    let argumets = Actions::get_arguments(index);
    println!("argumets = {:?}", argumets);

    let types = Actions::get_types(index);
    println!("types = {:?}", types);
}
