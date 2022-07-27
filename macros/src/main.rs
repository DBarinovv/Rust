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
            fn get_fields() -> Vec<String> {
                vec![$(stringify!($field_name).to_string(), ) *]
            }

            fn get_field(index: usize) -> String
            {
                Self::get_fields()[index].clone()
            }

            fn get_arguments(start_ind: usize, end_ind: usize) -> Vec<String> { 
                let mut res = Vec::new();
                $(res.append(&mut vec![$(stringify!($var_name).to_string()), *]);)*
                (&res[start_ind..end_ind]).to_vec()
            }

            fn get_size(index: usize) -> usize {
                let mut res = Vec::new();
                $(res.append(&mut vec![$crate::count![@COUNT; $($var_name), *]]);)*
                res[index]
            }

            fn get_types() -> Vec<String> {
                let mut res = Vec::new();
                $(res.append(&mut vec![$(stringify!($var_type).to_string()), *]);)*
                res
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:ident),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:ident) => { () };
}

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
    let fields = Actions::get_field(1);
    println!("fields = {:?}", fields);

    let argumets = Actions::get_arguments(0, 2);
    println!("argumets = {:?}", argumets);

    let size = Actions::get_size(0);
    println!("Size = {:?}", size);

    let types = Actions::get_types();
    println!("types = {:?}", types);
}
