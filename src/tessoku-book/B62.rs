use proconio::input;
use proconio::marker::Chars;
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MAX: i64 = 1 << 62;

// bps（最短経路の復元）
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize); m],
    }
    let mut root = vec![vec![]; n];
    for &(a, b) in &ab {
        root[a - 1].push(b - 1);
        root[b - 1].push(a - 1);
    }
    let mut dist = vec![MAX; n];
    dist[0] = 0;
    let mut stack: VecDeque<usize> = VecDeque::new();
    stack.push_back(0);
    while !stack.is_empty() {
        let now = stack.pop_front().unwrap();
        for &next in &root[now] {
            if dist[next] == MAX {
                stack.push_back(next);
                dist[next] = dist[now] + 1;
            }
        }
    }
    let mut ans = vec![n];
    let mut now = n - 1;
    while now != 0 {
        for &next in &root[now] {
            if dist[now] == dist[next] + 1 {
                ans.push(next + 1);
                now = next;
                break;
            }
        }
    }
    ans.reverse();
    // 配列の文字列化
    let ans_rev = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans_rev);
}
