use proconio::input;
use proconio::marker::Chars;

// dp LCS(最長共通部分列)
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let s_len = s.len();
    let t_len = t.len();
    let mut dp = vec![vec![0; t_len + 1]; s_len + 1];

    for i in 0..=s_len {
        dp[i][0] = i;
    }
    for j in 0..=t_len {
        dp[0][j] = j;
    }
    for i in 1..=s_len {
        for j in 1..=t_len {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]) + 1;
            }
        }
    }
    println!("{}", dp[s_len][t_len]);
}
