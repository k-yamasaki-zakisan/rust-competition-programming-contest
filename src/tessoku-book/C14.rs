use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MOD_I64: i64 = 1000000007;
const INF: usize = 1 << 62;
const INF_MINUS: i64 = -1000000007;

// 両端からダイクストラ法
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize,usize,usize); m],
    }
    let mut costs: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    let mut costs_copy: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for &(a, b, c) in &abc {
        costs[a - 1].push((b - 1, c));
        costs[b - 1].push((a - 1, c));
        costs_copy[a - 1].push((b - 1, c));
        costs_copy[b - 1].push((a - 1, c));
    }
    let zero_to_n = dij(costs, 0);
    let n_to_zero = dij(costs_copy, n - 1);
    let mut ans = 0;
    for i in 0..n {
        if zero_to_n[i] + n_to_zero[i] == zero_to_n[n - 1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn dij(cost: Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let n = cost.len();
    let mut distance = vec![INF; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start));
    while let Some((Reverse(d), target)) = heap.pop() {
        if distance[target] < d {
            continue;
        }
        distance[target] = d;
        for &(next, cost) in &cost[target] {
            if distance[next] > d + cost {
                distance[next] = d + cost;
                heap.push((Reverse(distance[next]), next));
            }
        }
    }
    distance
}
