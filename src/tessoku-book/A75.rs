use proconio::input;

// dp
fn main() {
    input! {
        n: usize,
        mut tm: [(usize, usize); n],
    }
    let NONE_COUNT = -1;
    let MAX_TIME = 1440;
    let mut dp = [NONE_COUNT; 1441];
    dp[0] = 0;
    // 2重配列のソート
    tm.sort_by(|l, r| l.1.partial_cmp(&(r.1)).unwrap());
    // println!("{:?}", tm);
    for h in 0..n {
        let (t, m) = tm[h];
        for i in (0..MAX_TIME).rev() {
            let next_time = i + t;
            if dp[i] != NONE_COUNT && next_time <= m {
                dp[next_time] = dp[next_time].max(dp[i] + 1);
            }
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
