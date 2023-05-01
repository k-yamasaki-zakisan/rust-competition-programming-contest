use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;

// ソート
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    a.reverse();
    println!("{}", a[0] + a[1]);
}
