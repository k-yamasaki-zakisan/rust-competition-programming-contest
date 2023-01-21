use proconio::input;

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

// UnionFind
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }
    let mut uf = UnionFind::new(n);
    for (flag, s, t) in a {
        if flag == 1 {
            uf.unite(s - 1, t - 1);
        } else {
            let is_same = uf.is_same(s - 1, t - 1);
            println!("{}", if is_same { "Yes" } else { "No" })
        }
    }
}
