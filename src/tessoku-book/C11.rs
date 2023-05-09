use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;

// ２分探索（浮動小数の有効桁まで見る）
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [f64; n],
    }
    let mut l = 0.0;
    let mut r = 1000000000000.0;
    while 1e-6 < r - l {
        let mut mid = (l + r) * 0.5;
        let mut tmp = 0;
        for a_val in a.clone() {
            tmp += (a_val / mid) as i64;
        }
        if k <= tmp {
            l = mid;
        } else {
            r = mid;
        }
    }
    let mut cnts = vec![0; n];
    for i in 0..n {
        cnts[i] = (a[i] / l) as i64;
    }
    let ans = cnts
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
