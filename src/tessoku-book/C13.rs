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

//
fn main() {
    input! {
        n: usize,
        p: i64,
        a: [i64; n],
    }
    let mut memo = HashMap::new();
    let mut ans = 0;
    for i in 0..n {
        let a = a[i] % MOD_I64;
        if a == 0 && p == 0 {
            ans += i
        } else {
            let tmp = ((p * mod_pow(a, MOD_I64 - 2, MOD_I64)) % MOD_I64) as usize;
            if let Some(num) = memo.get(&tmp) {
                ans += num
            }
        }
        *memo.entry(a as usize).or_insert(0) += 1;
    }
    println!("{}", ans);
}

fn mod_pow(a: i64, b: i64, m: i64) -> i64 {
    let mut p = a;
    let mut i = 0;
    let mut ans = 1;
    while 1 << i <= b {
        if b >> i & 1 == 1 {
            ans = (ans * p) % m;
        }
        p = (p * p) % m;
        i += 1
    }
    ans
}
