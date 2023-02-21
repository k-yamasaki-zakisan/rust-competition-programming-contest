use proconio::input;

// DP（復元）
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![-1; k + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..=k {
            if dp[i][j] != -1 {
                dp[i + 1][j] = dp[i][j];
                if j + a[i] <= k {
                    dp[i + 1][j + a[i]] = i as i32;
                }
            }
        }
    }

    if dp[n][k] == -1 {
        println!("-1");
        return;
    }
    let mut now = k;
    let mut ans = vec![];
    for i in (0..n).rev() {
        if dp[i + 1][now] != (i) as i32 {
            continue;
        }
        if now < a[i] {
            break;
        }
        now -= a[i];
        ans.push(i + 1);
    }
    ans.reverse();
    let ans_str = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans.len());
    println!("{}", ans_str);
}
