use proconio::input;

// ２次元累積和
fn main() {
    input! {
        h: usize,
        w: usize,
        x:[[usize; w]; h],
        q : i32,
        abcd: [(usize, usize, usize, usize); q],
    }
    let mut cum = vec![vec![0; w + 1]; h + 1];
    cum[0][0] = x[0][0];

    for i in 1..h {
        cum[i][0] = cum[i - 1][0] + x[i][0];
    }
    for i in 1..w {
        cum[0][i] = cum[0][i - 1] + x[0][i];
    }

    for i in 1..h {
        for j in 1..w {
            cum[i][j] = cum[i - 1][j] + cum[i][j - 1] - cum[i - 1][j - 1] + x[i][j];
        }
    }

    for (a, b, c, d) in abcd {
        let mut ans = cum[c - 1][d - 1];
        if 1 < a {
            ans -= cum[a - 2][d - 1]
        }
        if 1 < b {
            ans -= cum[c - 1][b - 2]
        }
        if 1 < a && 1 < b {
            ans += cum[a - 2][b - 2]
        }
        println!("{}", ans)
    }
}
