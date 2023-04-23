use proconio::input;
use proconio::marker::Chars;
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MAX: usize = 1 << 62;
const INF: usize = 1 << 62;

// dfs
fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(usize, usize); n-1],
    }
    let mut adj_list = vec![vec![]; n + 1];
    for &(a, b) in ab.iter() {
        adj_list[a - 1].push(b - 1);
        adj_list[b - 1].push(a - 1);
    }

    // dfsで階級を計算する
    let mut ranks = vec![INF; n];
    let mut visited = vec![false; n];
    dfs(&mut ranks, &mut visited, &adj_list, t - 1);

    let ans = ranks
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    // 結果の出力
    println!("{}", ans);
}

fn dfs(
    ranks: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    adj_list: &Vec<Vec<usize>>,
    node: usize,
) -> usize {
    let mut rank = 0;
    visited[node] = true;
    for &next in adj_list[node].iter() {
        if visited[next] {
            continue;
        }
        rank = dfs(ranks, visited, adj_list, next).max(rank);
    }
    ranks[node] = rank;
    return rank + 1;
}
