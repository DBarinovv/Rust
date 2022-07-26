use tuple_conv::*;
use core::mem::size_of;

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

macro_rules! get_info {
    ($enum_vis:vis enum $enum_name:ident { $( $(patt:meta)? $field_name:ident { $($fname:ident: $ftype:ty $(,)?)* } ,)* }) => {
        $enum_vis enum $enum_name { 
            $( $field_name { 
                $($fname: $ftype,)* 
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

            fn get_arguments(start_ind: usize, end_ind: usize) -> Vec<String>{
                let tuple = ($($(stringify!($fname).to_string(), ) *) *);
                let res = &tuple.to_vec()[start_ind..end_ind];
                res.to_vec()
            }

            fn get_size(name: &str) {
                
            }
        }
    }
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

    Actions::get_size("CreateMe");
}
