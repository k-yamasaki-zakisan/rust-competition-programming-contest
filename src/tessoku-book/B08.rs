use proconio::input;

// ２次元累積和
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    let h = 1510;
    let w = 1510;
    let mut cum = vec![vec![0; w]; h];
    for (x, y) in xy.into_iter() {
        cum[x][y] += 1;
    }
    for i in 0..h {
        for j in 1..w {
            cum[i][j] += cum[i][j - 1];
        }
    }
    for j in 0..w {
        for i in 1..h {
            cum[i][j] += cum[i - 1][j];
        }
    }
    for (a, b, c, d) in abcd.into_iter() {
        let ans = cum[c][d] + cum[a - 1][b - 1] - cum[c][b - 1] - cum[a - 1][d];
        println!("{}", ans);
    }
}
