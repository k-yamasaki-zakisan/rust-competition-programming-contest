use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: usize = 1000000007;

//
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut ans = vec![m; n];
    for a_val in a {
        ans[a_val - 1] -= 1;
    }
    for ans_val in ans {
        println!("{}", ans_val);
    }
}
