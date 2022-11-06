use proconio::input;

// dp easy
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }
    let inf = 1 << 60;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = dp[i - 1] + a[i - 1];
        if 1 < i {
            dp[i] = dp[i].min(dp[i - 2] + b[i - 2]);
        }
    }
    println!("{}", dp[n - 1]);
}
