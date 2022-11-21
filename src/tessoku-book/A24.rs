use proconio::input;

// LIS（最長増加部分列）
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp = vec![a[0]];
    for a_val in a {
        let i = bisect(&dp, a_val);
        if i == dp.len() {
            dp.push(a_val)
        } else {
            dp[i] = a_val;
        }
    }
    println!("{}", dp.len());
}

fn bisect(a: &Vec<usize>, target: usize) -> usize {
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
