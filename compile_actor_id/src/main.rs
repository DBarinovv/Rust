#![feature(int_log)]

mod actor_id_prototype;
use actor_id_prototype::ActorId;

use rand::Rng;

const U64_VAR: u64 = 12345;
const COMPILE_ACTOR: ActorId = make_actor(U64_VAR);

const fn make_actor(val: u64) -> ActorId {
    if val == 0 {
        return ActorId::zero()
    }

    let arr = [0; 32];
    let res = get_actor_id(arr, val.log(256) as usize, val);
    ActorId::new(res)
}
const fn get_actor_id(mut actor_id: [u8; 32], i: usize, val: u64) -> [u8; 32] {
    actor_id[i] = (val / 256_u64.pow(i as u32)) as u8;

    if i == 0 {
        return actor_id;
    }

    return get_actor_id(actor_id, i - 1, val % 256_u64.pow(i as u32));
}

// run with 
// cargo +nightly test
fn main() {}

#[test]
fn simple_test() {
    assert_eq!(ActorId::from(U64_VAR), COMPILE_ACTOR);
}

#[test]
fn zero() {
    assert_eq!(ActorId::zero(), make_actor(0));
}

#[test] 
fn u64_max() {
    assert_eq!(ActorId::from(u64::MAX), make_actor(u64::MAX))
}

#[test]
fn rand_var() {
    for i in 0..100000 {
        let var: u64 = rand::thread_rng().gen_range(0..u64::MAX);
        assert_eq!(ActorId::from(var), make_actor(var));
    }
}

#[test]
fn consecutive_numbers() {
    for var in 0..1000 {
        assert_eq!(ActorId::from(var), make_actor(var), "msg");
    }

    for var in 1000..2000 {
        assert_eq!(ActorId::from(var), make_actor(var));
    }

    for var in 100000..101000 {
        assert_eq!(ActorId::from(var), make_actor(var));
    }
}
