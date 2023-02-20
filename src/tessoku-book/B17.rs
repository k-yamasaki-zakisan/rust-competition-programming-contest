use proconio::input;

// DP（復元）
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let inf = 10000000000;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        if i + 1 < n {
            dp[i + 1] = dp[i + 1].min(dp[i] + (a[i] - a[i + 1]).abs())
        }
        if i + 2 < n {
            dp[i + 2] = dp[i + 2].min(dp[i] + (a[i] - a[i + 2]).abs())
        }
    }
    let mut now = n - 1;
    let mut ans = vec![n];
    while 0 < now {
        for i in 1..=2 {
            // if now - i < 0 {
            //     break;
            // }
            if dp[now] - dp[now - i] == (a[now] - a[now - i]).abs() {
                now = now - i;
                ans.push(now + 1);
                break;
            }
        }
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
