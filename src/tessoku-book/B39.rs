use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: usize = 1000000007;

// heapで取り出し続ける
fn main() {
    input! {
        n: usize,
        d: usize,
        mut xy: [(usize, usize); n],
    }
    xy.sort_by(|l, r| l.0.partial_cmp(&(r.0)).unwrap());
    // println!("{:?}", xy);
    let mut heap = BinaryHeap::new();
    let mut now = 0;
    let mut ans: usize = 0;
    for day in 1..=d {
        while now < n && xy[now].0 == day {
            heap.push(xy[now].1);
            now += 1;
        }
        if 0 < heap.len() {
            while let Some(cost) = heap.pop() {
                ans += cost;
                break;
            }
        }
    }
    println!("{}", ans);
}
