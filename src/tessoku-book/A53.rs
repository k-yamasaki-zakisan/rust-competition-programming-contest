use proconio::input;
use std::collections::BinaryHeap;

// heapq
fn main() {
    input! {
        q: usize,
    }
    let mut memo: BinaryHeap<i32> = BinaryHeap::new();
    for _ in 0..q {
        input! {
            x: usize
        }
        if x == 1 {
            input! {
                y: i32
            }
            memo.push(-y);
        } else if x == 2 {
            println!("{}", -(*memo.peek().unwrap()));
        } else {
            memo.pop().unwrap();
        }
    }
}
