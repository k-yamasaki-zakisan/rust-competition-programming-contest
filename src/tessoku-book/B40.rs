use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: usize = 1000000007;

// mod
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut memo: Vec<usize> = vec![0; 100];
    let mut ans = 0;
    for a_val in a {
        let tmp = a_val % 100;
        ans += memo[(100 - tmp) % 100];
        memo[tmp] += 1;
    }
    println!("{}", ans);
}
