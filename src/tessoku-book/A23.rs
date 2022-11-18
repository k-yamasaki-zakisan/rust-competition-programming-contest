use proconio::input;

// bitDP
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }
    let inf = 1_000;
    let len_n = 1 << n;
    let mut dp = vec![vec![inf; 1 << n]; m + 1];
    dp[0][0] = 0;
    for i in 1..=m {
        for bit in 0..len_n {
            if dp[i - 1][bit] == inf {
                continue;
            }
            dp[i][bit] = dp[i][bit].min(dp[i - 1][bit]);
            let mut tmp = 0;
            for j in 0..n {
                tmp += a[i - 1][j] << j;
            }
            dp[i][bit | tmp] = dp[i][bit | tmp].min(dp[i - 1][bit] + 1);
        }
    }
    println!(
        "{}",
        if dp[m][len_n - 1] == inf {
            -1
        } else {
            dp[m][len_n - 1]
        }
    );
}
