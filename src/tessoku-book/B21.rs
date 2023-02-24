use proconio::input;
use proconio::marker::Chars;

// dp（文字列から複数除去、回分作成）
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut t = vec![];
    for i in (0..n).rev() {
        t.push(s[i]);
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            let is_same = if s[i - 1] == t[j - 1] { 1 } else { 0 };
            dp[i][j] = dp[i - 1][j]
                .max(dp[i][j - 1])
                .max(dp[i - 1][j - 1] + is_same);
        }
    }
    println!("{}", dp[n][n]);
}
