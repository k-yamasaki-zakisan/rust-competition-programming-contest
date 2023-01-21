use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

const INF: usize = 1 << 31;

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

// ダイクストラ法+heap
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize, usize); m],
    }
    let mut root: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for &(a, b, c) in &ab {
        root[a - 1].push((b - 1, c));
        root[b - 1].push((a - 1, c));
    }
    let result = dij(root, 0);
    for r in result {
        if r == INF {
            println!("{}", -1);
        } else {
            println!("{}", r);
        }
    }
}
