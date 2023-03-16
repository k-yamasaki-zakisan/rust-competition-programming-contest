use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap};

const MOD: usize = 1000000007;

//
fn main() {
    input! {
        a: Chars,
    }
    let mut left: Vec<usize> = vec![];
    for i in 0..a.len() {
        if a[i] == '(' {
            left.push(i + 1);
        } else {
            println!("{} {}", left.pop().unwrap(), i + 1);
        }
    }
}
