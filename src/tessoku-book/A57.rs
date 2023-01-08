use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
        xy: [(Usize1, usize); q]
    }
    let mut dp = vec![vec![0; n]; 30];
    for i in 0..n {
        dp[0][i] = a[i];
    }
    for i in 1..30 {
        for j in 0..n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }
    for (x, y) in xy {
        let mut pos = x;
        for i in (0..30).rev() {
            if y & (1 << i) != 0 {
                pos = dp[i][pos];
            }
        }
        println!("{}", pos + 1);
    }
}
