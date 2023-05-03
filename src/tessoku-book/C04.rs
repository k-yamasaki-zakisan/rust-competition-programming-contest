use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;

// 1~平方数を割れるかの判定
fn main() {
    input! {
        d: i64,
    }
    let mut memo = vec![];
    memo.push(1);
    memo.push(d);
    // 平方根の処理
    for num in 2..((d as f64).sqrt() as i64) + 1 {
        if d % num == 0 {
            memo.push(num);
            memo.push(d / num);
        }
    }
    memo.sort();
    for a in memo {
        println!("{}", a);
    }
}
