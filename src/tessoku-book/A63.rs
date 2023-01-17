use proconio::input;
use std::collections::VecDeque;

// bps
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut root = vec![vec![]; n];
    for &(a, b) in &ab {
        root[a - 1].push(b - 1);
        root[b - 1].push(a - 1);
    }
    let mut visited = vec![-1; n];
    visited[0] = 0;
    let mut stack: VecDeque<usize> = VecDeque::new();
    stack.push_back(0);
    while !stack.is_empty() {
        let now = stack.pop_front().unwrap();
        for &next in &root[now] {
            if visited[next] == -1 {
                stack.push_back(next);
                visited[next] = visited[now] + 1;
            }
        }
    }
    for v in visited {
        println!("{}", v);
    }
}
