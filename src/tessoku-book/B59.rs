use proconio::input;
use proconio::marker::Chars;
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MAX: i64 = 1 << 62;

struct SegmentTree {
    arr: Vec<i64>,
    size: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        Self {
            arr: vec![0; 2 * size],
            size,
        }
    }
    fn size(&self) -> usize {
        self.size
    }
    fn update(&mut self, pos: usize, x: i64) {
        let mut p = self.size - 1 + pos;
        self.arr[p] = x;
        while p >= 2 {
            p /= 2;
            self.arr[p] = self.arr[2 * p] + self.arr[2 * p + 1]
        }
    }
    fn query(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> i64 {
        if a >= r || b <= l {
            return 0;
        }
        if a >= l && b <= r {
            return self.arr[u];
        }
        let left_child = self.query(l, r, a, (a + b) / 2, 2 * u);
        let right_child = self.query(l, r, (a + b) / 2, b, 2 * u + 1);
        left_child + right_child
    }
}

// SegmentTree
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut result: i64 = 0;
    let mut stree = SegmentTree::new(n + 10);
    for i in 0..n {
        let v = a[i];
        stree.update(v, 1);

        let sum = stree.query(0, v, 1, stree.size() + 1, 1);
        let nv = i as i64 - sum;

        result += nv;
    }
    println!("{}", result);
}
