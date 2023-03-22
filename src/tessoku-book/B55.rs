use proconio::input;
use proconio::marker::Chars;
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MAX: i64 = 1 << 62;

// 並行二分木探索
fn main() {
    input! {
        q: usize,
        qs: [(usize, i64); q],
    }
    let mut set = BTreeSet::new();
    for (q_1, q_2) in qs {
        if q_1 == 1 {
            set.insert(q_2);
        } else {
            let mut ans: i64 = MAX;
            if let Some(&v) = set.range(0..=q_2).rev().next() {
                ans = min(ans, q_2 - v);
            }

            if let Some(&v) = set.range(q_2..).next() {
                ans = min(ans, v - q_2);
            }
            println!("{}", if ans == MAX { -1 } else { ans });
        }
    }
}
