use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;

// ２進数をうまく使えば簡単に解けそう
fn main() {
    input! {
        d: usize,
    }
    let mut memo: VecDeque<String> = VecDeque::new();
    memo.push_back("".to_string());
    let mut cnt = 0;
    while true {
        let tmp = memo.pop_front().unwrap();
        if tmp.len() == 10 {
            cnt += 1;
        }
        if cnt == d {
            println!("{}", tmp);
            return;
        }
        memo.push_back(tmp.clone() + "4");
        memo.push_back(tmp.clone() + "7");
    }
}
