use proconio::input;
use proconio::marker::Usize1;

// dp
fn main() {
    input! {
        n: usize,
        pa: [(Usize1, usize); n],
    }
    let mut dp = vec![vec![0; n]; n];
    for i in 0..(n - 1) {
        for j in (i + 1..n).rev() {
            let (p, a) = &pa[i];
            let plus = if i <= *p && *p <= j { *a } else { 0 };
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + plus);
            let (p, a) = &pa[j];
            let plus = if i <= *p && *p <= j { *a } else { 0 };
            dp[i][j - 1] = dp[i][j - 1].max(dp[i][j] + plus);
        }
    }
    println!("{}", (0..n).map(|i| dp[i][i]).max().unwrap());
}
