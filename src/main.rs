#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

fn main() {
    let num : u64 = 7777777777777777777;

    // Pretty print
    println!("num = {} ", mem::size_of_val(&num));
}

