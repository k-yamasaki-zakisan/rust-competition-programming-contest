use proconio::input;

// dp（巡回セールスマン）
fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut cost: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        let (x_1, y_1) = xy[i];
        for j in 0..n {
            let (x_2, y_2) = xy[j];
            cost[i][j] = ((x_1 - x_2) * (x_1 - x_2) + (y_1 - y_2) * (y_1 - y_2)).sqrt();
        }
    }

    let inf: f64 = 10000000000000.0;
    let mut dp: Vec<Vec<f64>> = vec![vec![inf; n]; (1 << n)];

    dp[0][0] = 0.0;
    for bit in 1..(1 << n) {
        for t in 0..n {
            if bit & (1 << t) > 0 {
                for k in 0..n {
                    dp[bit][t] = dp[bit][t].min(dp[bit - (1 << t)][k] + cost[k][t]);
                }
            }
        }
    }

    let ans: f64 = if dp[(1 << n) - 1][0] == inf {
        -1.0
    } else {
        dp[(1 << n) - 1][0] as f64
    };

    println!("{}", ans);
}
