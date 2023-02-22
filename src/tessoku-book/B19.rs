use proconio::input;

// DP
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let wide = 1000 * (n + 1);
    let inf = std::usize::MAX;
    let mut dp = vec![vec![inf; wide + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (w, v) = wv[i];
        for j in 0..=wide {
            if dp[i][j] == inf {
                continue;
            }
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            dp[i + 1][j + v] = dp[i + 1][j + v].min(dp[i][j] + w);
        }
    }
    let mut ans = 0;
    for j in 0..=wide {
        if dp[n][j] <= w {
            ans = j;
        }
    }
    println!("{}", ans);
}
