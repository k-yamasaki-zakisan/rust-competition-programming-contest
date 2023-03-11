use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: usize = 1000000007;

// よくわからんが勘で解いた
fn main() {
    input! {
        mut x: usize,
        mut y: usize,
    }
    let mut ans = vec![(x, y)];
    while x != 1 || y != 1 {
        if x < y {
            y -= x;
        } else {
            x -= y;
        }
        ans.push((x, y));
    }
    ans.pop();
    ans.reverse();
    println!("{}", ans.len());
    for &(a, b) in &ans {
        println!("{} {}", a, b);
    }
}
