use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MAX: usize = 1 << 62;
const INF: usize = 1 << 62;

// UnionFind
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        q: usize,
    }
    let mut queries = vec![];
    let mut drops = vec![];
    for _ in 0..q {
        input! {
            f: usize,
        }
        if f == 1 {
            input! {
                s: usize,
            }
            queries.push((1, s - 1, INF));
            drops.push(s - 1);
        } else {
            input! {
                s: usize,
                t: usize,
            }
            queries.push((2, s - 1, t - 1));
        }
    }
    let mut ut = UnionFind::new(n);
    for i in 0..m {
        if !drops.iter().any(|&x| x == i) {
            ut.unite(ab[i].0, ab[i].1);
        }
    }
    let mut ans = vec![];
    queries.reverse();
    for (target, s, t) in queries {
        if target == 1 {
            ut.unite(ab[s].0, ab[s].1);
        } else {
            if ut.is_same(s, t) {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
        }
    }
    ans.reverse();
    for a in ans {
        println!("{}", a);
    }
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}
