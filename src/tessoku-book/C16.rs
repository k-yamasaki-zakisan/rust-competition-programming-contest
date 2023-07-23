use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MOD_I64: i64 = 1000000007;
const INF: usize = 1 << 62;
const INF_MINUS: i64 = -1000000007;

// dps
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        asbt: [(usize,usize,usize,usize); m],
    }
    let mut heap = BinaryHeap::new();
    for (a, s, b, t) in asbt.clone() {
        heap.push((Reverse(s * 10 + 1), a - 1, b - 1, (t + k) * 10));
    }
    let mut dp = vec![0; n];
    while let Some((Reverse(s), a, b, t)) = heap.pop() {
        let time = s / 10;
        let tf = s % 10;
        if tf != 0 {
            heap.push((Reverse(t), a, b, dp[a]));
        } else {
            dp[b] = dp[b].max(t + 1)
        }
    }
    let mut ans = 0;
    for d in dp {
        ans = ans.max(d);
    }
    println!("{}", ans);
}
