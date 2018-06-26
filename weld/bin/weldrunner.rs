extern crate weld;

use weld::*;
use std::env;

#[path="../../tests/common/mod.rs"]
mod common;
use common::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 {
        // Set up code
        // "expr" i.e., |v: vec[i32]| expr"
        let expr: &str = &args[1];
        let mut code: String = "|v: vec[i32]| ".to_owned();
        code.push_str(expr);

        // Set up vec of i32's
        let input = &args[2];
        let input_vec: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Set up Weld and run
        let ref input_data = WeldVec::from(&input_vec);
        let ref conf = default_conf();
        let ret_value = compile_and_run(&code, conf, input_data);
        let data = ret_value.data() as *const WeldVec<i32>;
        let result = unsafe { (*data).clone() };
        for i in 0..(result.len as isize) {
            if i == (result.len -1) as isize {
                print!("{}]\n", unsafe { *result.data.offset(i)})
            } else if i == 0 {
                print!("[{},", unsafe { *result.data.offset(i)})
            } else {
                print!("{},", unsafe { *result.data.offset(i)})
            }
        }
    } else {
        println!("EXPR VEC: \"map()\" \"10 20 30\"");
        println!("./weldrunner \"map(v, |a:i32| a + i32(10))\" \"10 20 30 40\"");
    }

}
