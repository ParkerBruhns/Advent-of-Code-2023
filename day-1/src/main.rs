#![allow(unused)]

mod try_one;
mod try_two;

use crate::try_one::*;
use crate::try_two::*;

fn main() {
    try_one();
    try_two();
}

fn _test_contains() {
    let str = "Parker";
    if str.contains("pa") {
        println!("True");
    } else {
        println!("False");
    }
}
