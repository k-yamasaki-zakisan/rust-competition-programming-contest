use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap, collections::VecDeque};

const MOD: usize = 1000000007;

// それまでの文字列の個数をメモっておく
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut hash_map = HashMap::new();
    let mut ans: i64 = 0;
    for a_val in a {
        if hash_map.contains_key(&a_val) {
            ans += hash_map[&a_val];
        }
        *hash_map.entry(a_val).or_insert(0) += 1;
    }
    println!("{}", ans);
}
