use proconio::input;
use std::collections::BTreeSet;

// binary tree
fn main() {
    input! {
        q: usize,
    }
    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! {
            x: usize,
            y: usize
        }
        if x == 1 {
            set.insert(y);
        } else if x == 2 {
            set.remove(&y);
        } else if x == 3 {
            if let Some(&y) = set.range(y..).next() {
                println!("{}", y);
            } else {
                println!("-1");
            }
        }
    }
}
