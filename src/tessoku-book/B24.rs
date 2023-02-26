use proconio::input;

// ソート+最長増加部分列（２分探索）
fn main() {
    input! {
        n: usize,
        mut xy: [(i32, i32); n],
    }
    // 2重配列のソート（x昇順, y降順）
    xy.sort_by_cached_key(|&(x, y)| (x, -y));
    let mut dp: Vec<i32> = vec![xy[0].1];
    for (x, y) in xy {
        let i: usize = bisect(&dp, y);
        if i == dp.len() {
            dp.push(y)
        } else {
            dp[i] = y;
        }
    }
    println!("{}", dp.len());
}

fn bisect(a: &Vec<i32>, target: i32) -> usize {
    let mut l = -1;
    let mut r = a.len() as i64;
    while 1 < r - l {
        let ii = (l + r) / 2;
        if a[ii as usize] < target {
            l = ii;
        } else {
            r = ii;
        }
    }
    (l + 1) as usize
}
