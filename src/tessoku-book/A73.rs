use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

const INF: usize = 1 << 31;

fn dij(cost: Vec<Vec<(usize, usize, usize)>>, start: usize) -> Vec<usize> {
    let n = cost.len();
    let mut distance = vec![INF; n];
    let mut trees = vec![0; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start));
    while let Some((Reverse(d), target)) = heap.pop() {
        if distance[target] < d {
            continue;
        }
        distance[target] = d;
        for &(next, cost, tree) in &cost[target] {
            if distance[next] > d + cost {
                distance[next] = d + cost;
                heap.push((Reverse(distance[next]), next));
                trees[next] = trees[target] + tree;
            } else if distance[next] == d + cost && trees[next] < trees[target] + tree {
                trees[next] = trees[target] + tree;
            }
        }
    }
    let mut ans = vec![];
    ans.push(distance.pop().unwrap());
    ans.push(trees.pop().unwrap());
    return ans;
}

// ダイクストラ法+heap
fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(usize, usize, usize, usize); m],
    }
    let mut root: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; n];
    for &(a, b, c, d) in &abcd {
        root[a - 1].push((b - 1, c, d));
        root[b - 1].push((a - 1, c, d));
    }
    let result = dij(root, 0);
    println!("{} {}", result[0], result[1]);
}
