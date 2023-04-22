use proconio::input;
use proconio::marker::Chars;
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MAX: i64 = 1 << 62;

// bps（最短経路）
fn main() {
    input! {
        r: usize,
        c: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
        ab: [Chars; r],
    }
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    stack.push_back((sy - 1, sx - 1));
    let mut dp = vec![vec![MAX; c]; r];
    dp[sy - 1][sx - 1] = 0;
    while !stack.is_empty() {
        let (now_h, now_w) = stack.pop_front().unwrap();
        let next = [
            (now_h + 1, now_w),
            (now_h, now_w + 1),
            (now_h - 1, now_w),
            (now_h, now_w - 1),
        ];
        for &(next_h, next_w) in next.iter() {
            if 0 <= next_h
                && next_h < r
                && 0 <= next_w
                && next_w < c
                && ab[next_h][next_w] == '.'
                && dp[next_h][next_w] == MAX
            {
                stack.push_back((next_h, next_w));
                dp[next_h][next_w] = dp[now_h][now_w] + 1;
            }
        }
    }
    println!("{}", dp[gy - 1][gx - 1]);
}
