use proconio::input;
use std::collections::VecDeque;

// queue
fn main() {
    input! {
        q: usize,
    }
    let mut memo: VecDeque<String> = VecDeque::new();
    for _ in 0..q {
        input! {
            x: usize
        }
        if x == 1 {
            input! {
                y: String
            }
            memo.push_back(y);
        } else if x == 2 {
            println!("{}", memo.front().unwrap());
        } else {
            memo.pop_front().unwrap();
        }
    }
}
