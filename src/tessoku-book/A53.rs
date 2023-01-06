use proconio::input;
use std::collections::HashMap;

// heapq
fn main() {
    input! {
        q: usize,
    }
    let mut memo: HashMap<String, usize> = HashMap::new();
    for _ in 0..q {
        input! {
            x: usize,
            name: String
        }
        if x == 1 {
            input! {
                y: usize
            }
            memo.insert(name, y);
        } else {
            println!("{}", memo[&name]);
        }
    }
}
