use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;
const INF_MINUS: i64 = -1000000007;

// dp（指定数分割したときのある指標の最大化）
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); m],
    }
    let mut dp = vec![vec![INF_MINUS; k + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..n + 1 {
        dp[i] = dp[i - 1].clone();
        for j in 1..k + 1 {
            for l in 0..i {
                let mut cnt = 0;
                for &(a, b) in &ab {
                    if l < a && b <= i {
                        cnt += 1
                    }
                }
                dp[i][j] = dp[i][j].max(dp[l][j - 1] + cnt)
            }
        }
    }
    println!("{}", dp[n][k]);
}
