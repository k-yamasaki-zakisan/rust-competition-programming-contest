use proconio::input;

// dp ナップサック
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut dp = vec![vec![0; w + 1]; n + 1];

    dp[0][0] = 0;
    for (h, (wid, val)) in wv.iter().enumerate() {
        for j in 0..=w {
            dp[h + 1][j] = dp[h + 1][j].max(dp[h][j]);
            if j + wid <= w {
                dp[h + 1][j + wid] = dp[h + 1][j + wid].max(dp[h][j] + val)
            }
        }
    }
    let mut ans = 0;
    for d in dp.iter() {
        ans = ans.max(*d.iter().max().unwrap());
    }
    println!("{}", ans);
}
