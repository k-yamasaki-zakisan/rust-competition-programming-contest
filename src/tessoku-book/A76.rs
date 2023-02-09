use proconio::input;

const MOD: i64 = 1000000007;

//  dp && 累積和 && ２分探索
fn main() {
    input! {
        n: usize,
        w: usize,
        l: usize,
        r: usize,
        x: [usize; n],
    }
    let max_n = n + 2;
    let mut points = vec![0];
    for xx in x {
        points.push(xx);
    }
    points.push(w);
    let mut dp = vec![0; max_n + 1];
    dp[0] = 1;
    dp[1] = -1;
    for i in 0..max_n - 1 {
        let now_point = points[i];
        let next_min_point = now_point + l - 1;
        let mut l_i = 0;
        let mut r_i = max_n;
        while 1 < r_i - l_i {
            let ii = (l_i + r_i) / 2;
            if points[ii] <= next_min_point {
                l_i = ii;
            } else {
                r_i = ii;
            }
        }
        dp[r_i] += dp[i];
        dp[r_i] %= MOD;

        let next_max_point = now_point + r;
        let mut l_i = 0;
        let mut r_i = max_n;
        while 1 < r_i - l_i {
            let ii = (l_i + r_i) / 2;
            if points[ii] <= next_max_point {
                l_i = ii;
            } else {
                r_i = ii;
            }
        }
        dp[r_i] -= dp[i];
        dp[r_i] %= MOD;
        dp[i + 1] += dp[i];
        dp[i + 1] %= MOD;
    }
    println!("{}", dp[n + 1] % MOD);
}
