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

    dp[0][0] = 0;
    for i in 1..=s_len {
        for j in 1..=t_len {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
            }
        }
    }
    // ２重配列の要素の最大数を取得
    let mut ans = 0;
    for d in dp.iter() {
        ans = ans.max(*d.iter().max().unwrap());
    }
    println!("{}", ans);
}
