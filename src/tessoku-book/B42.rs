use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: usize = 1000000007;

// 3年前全く同じ問題を初見で解けなかった...（懐かしい、１０秒で考察できて成長を感じた）
fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut ans = 0;
    for &(f_s, s_s) in [('+', '+'), ('+', '-'), ('-', '+'), ('-', '-')].iter() {
        let mut tmp_sum = 0;
        for &(a, b) in &ab {
            let mut tmp = 0;
            tmp += if f_s == '+' { a } else { -a };
            tmp += if s_s == '+' { b } else { -b };
            if 0 < tmp {
                tmp_sum += tmp;
            }
        }
        ans = ans.max(tmp_sum);
    }
    println!("{}", ans);
}
