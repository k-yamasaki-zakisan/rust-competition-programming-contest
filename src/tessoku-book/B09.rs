use proconio::input;

// ２次元累積和
fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize,usize,usize); n],
    }
    let h = 1510;
    let w = 1510;
    let mut cum = vec![vec![0; w]; h];
    for (a, b, c, d) in abcd.clone().into_iter() {
        cum[a][b] += 1;
        cum[c][d] += 1;
        cum[a][d] -= 1;
        cum[c][b] -= 1;
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
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if 1 <= cum[i][j] {
                ans += 1
            }
        }
    }
    println!("{}", ans);
}
