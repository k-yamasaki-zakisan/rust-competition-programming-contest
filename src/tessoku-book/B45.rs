use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap};

const MOD: usize = 1000000007;

//
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    println!("{}", if a + b + c == 0 { "Yes" } else { "No" })
}
