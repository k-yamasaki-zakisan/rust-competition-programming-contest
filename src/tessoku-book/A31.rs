use proconio::input;

// dp
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut dp = vec![false; n + (a.max(b) + 1)];
    for i in 0..=n {
        if !dp[i] {
            dp[i + a] = true;
            dp[i + b] = true;
        }
    }
    println!("{}", if dp[n] { "First" } else { "Second" })
}
