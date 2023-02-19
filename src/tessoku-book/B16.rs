use proconio::input;

// DP
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let inf = 10000000000;
    let mut ans = vec![inf; n];
    ans[0] = 0;
    for i in 0..n - 1 {
        if i + 1 < n {
            ans[i + 1] = ans[i + 1].min(ans[i] + (a[i] - a[i + 1]).abs())
        }
        if i + 2 < n {
            ans[i + 2] = ans[i + 2].min(ans[i] + (a[i] - a[i + 2]).abs())
        }
    }
    println!("{}", ans.last().unwrap());
}
