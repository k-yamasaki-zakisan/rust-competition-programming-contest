use proconio::input;
use proconio::marker::Chars;
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MAX: i64 = 1 << 62;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize); m],
    }
    let mut memo = vec![0; n];
    for (a, b) in ab {
        memo[a - 1] += 1;
        memo[b - 1] += 1;
    }
    let friend_max = *memo.iter().max().unwrap();
    for i in 0..n {
        if memo[i] == friend_max {
            println!("{}", i + 1);
            return;
        }
    }
}
