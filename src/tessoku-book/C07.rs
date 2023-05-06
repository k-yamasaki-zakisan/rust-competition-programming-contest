use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;

// 累積和して２分探索
fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
        q: usize,
        x: [usize; q],
    }
    c.sort();
    let mut ruiseki = vec![0];
    for c_val in c {
        ruiseki.push(ruiseki[ruiseki.len() - 1] + c_val);
    }
    for x_val in x {
        println!("{}", bisect(&ruiseki, x_val));
    }
}

fn bisect(a: &Vec<usize>, target: usize) -> usize {
    let mut l = 0;
    let mut r = a.len();
    while 1 < r - l {
        let ii = (l + r) / 2;
        if a[ii] <= target {
            l = ii;
        } else {
            r = ii;
        }
    }
    l
}
