use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;

// メモ
fn main() {
    input! {
        d: usize,
        a: [i64; d],
        q: usize,
        st: [(Usize1, Usize1); q],
    }
    let mut memo = vec![];
    let mut tmp = 0;
    for a_val in a {
        tmp += a_val;
        memo.push(tmp)
    }
    for (s, t) in st {
        if (memo[s] < memo[t]) {
            println!("{}", t + 1);
        } else if (memo[t] < memo[s]) {
            println!("{}", s + 1);
        } else {
            println!("Same");
        }
    }
}
