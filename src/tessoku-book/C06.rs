use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;

//
fn main() {
    input! {
        d: usize,
    }
    println!("{}", d);
    for i in 1..d + 1 {
        if i == d {
            println!("{} {}", i, 1);
        } else {
            println!("{} {}", i, i + 1);
        }
    }
}
