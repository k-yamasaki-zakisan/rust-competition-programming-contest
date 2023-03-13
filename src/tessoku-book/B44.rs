use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap};

const MOD: usize = 1000000007;

// 移動した先を常にメモする
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        q: usize,
        qs: [(usize, usize, usize); q],
    }
    let mut memo: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        memo.insert(i, i);
    }
    for (q1, q2, q3) in qs {
        if (q1 == 1) {
            let x = memo[&(q2 - 1)];
            let y = memo[&(q3 - 1)];
            memo.insert(q2 - 1, y);
            memo.insert(q3 - 1, x);
        } else {
            println!("{}", a[memo[&(q2 - 1)]][q3 - 1]);
        }
    }
}
