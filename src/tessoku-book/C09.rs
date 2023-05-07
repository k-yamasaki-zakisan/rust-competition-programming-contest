use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;

// DP
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = vec![0, 0];
    for i in 2..n + 2 {
        ans.push(ans[i - 1].max(ans[i - 2] + a[i - 2]));
    }
    println!("{}", *ans.iter().max().unwrap());
}
