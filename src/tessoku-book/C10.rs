use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: i64 = 1000000007;
const INF: usize = 1 << 62;

// pow mod 繰り返し２乗法
fn main() {
    input! {
        n: i64,
    }
    let mut ans = (12 * mod_pow(7, n - 1, MOD)) % MOD;
    println!("{}", ans);
}

fn mod_pow(base: i64, exponent: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
}
