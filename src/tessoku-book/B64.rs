use proconio::input;
use proconio::marker::Chars;
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MAX: usize = 1 << 62;
const INF: usize = 1 << 62;

type Edges = Vec<Vec<(usize, usize)>>;

struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    pub fn new(n: usize, edges: Edges, init: usize) -> Self {
        let mut distance = vec![INF; n];
        let mut parent = vec![INF; n];
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), init));
        while let Some((Reverse(d), target)) = heap.pop() {
            if distance[target] < d {
                continue;
            }
            distance[target] = d;
            for &(next, cost) in &edges[target] {
                if distance[next] > d + cost {
                    distance[next] = d + cost;
                    heap.push((Reverse(distance[next]), next));
                    parent[next] = target;
                }
            }
        }
        Self { distance, parent }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }

    pub fn get_path(&self, target: usize) -> Vec<usize> {
        let mut current = target;
        let mut res = vec![current];
        while self.parent[current] != INF as usize {
            let next = self.parent[current];
            res.push(next);
            current = next;
        }
        res.reverse();
        res
    }
}

// ダイクストラ法:Dijkstra's
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }
    let mut edges: Edges = vec![vec![]; n];
    for &(a, b, c) in abc.iter() {
        edges[a - 1].push((b - 1, c));
        edges[b - 1].push((a - 1, c));
    }
    let graph = Dijkstra::new(n, edges, 0);
    let ans = graph
        .get_path(n - 1)
        .iter()
        .map(|x| (x + 1).to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
