use proconio::input;
use std::cmp::max;

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
            self.arr[p] = max(self.arr[2 * p], self.arr[2 * p + 1]);
        }
    }
    fn query(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> i64 {
        if a >= r || b <= l {
            return std::i64::MIN;
        }
        if a >= l && b <= r {
            return self.arr[u];
        }
        let left_child = self.query(l, r, a, (a + b) / 2, 2 * u);
        let right_child = self.query(l, r, (a + b) / 2, b, 2 * u + 1);
        return max(left_child, right_child);
    }
}

// SegmentTree（区間最大値）
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut tree = SegmentTree::new(n);
    for _ in 0..q {
        input! {
            query_id: usize,
        }
        if query_id == 1 {
            input! {
                pos: usize,
                x: i64
            }
            tree.update(pos, x);
        } else {
            input! {
                l: usize,
                r: usize
            }
            let ans = tree.query(l, r, 1, tree.size() + 1, 1);
            println!("{}", ans);
        }
    }
}
