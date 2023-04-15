use proconio::input;
// use proconio::marker::Chars;
// use std::{
//     cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
//     collections::VecDeque,
// };

const MOD: usize = 1000000007;
const MAX: i64 = 1 << 62;

//
fn main() {
    input! {
        n: i64,
        k: i64,
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![0; (n + 1) as usize]; 31];
    for i in 0..n + 1 {
        let mut sum_digit: i64 = 0;
        let mut num: i64 = i;
        while 0 < num {
            sum_digit += num % 10;
            num /= 10;
        }
        dp[0][i as usize] = i as i64 - sum_digit;
    }
    for mut i in 1..31 {
        for mut j in 0..=n as usize {
            dp[i][j] = dp[i - 1][dp[i - 1][j] as usize]
        }
    }
    for n in 1..n + 1 {
        let mut ans: i64 = n;
        for digit in 0..30 {
            if k & (1 << digit) != 0 {
                ans = dp[digit][ans as usize];
            }
        }
        println!("{}", ans);
    }
}
