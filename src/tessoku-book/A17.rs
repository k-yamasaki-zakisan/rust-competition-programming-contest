use proconio::input;

// dp easy
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }
    let mut dp = vec![0; n + 1];
    dp[2] = a[0];
    for i in 3..n + 1 {
        dp[i] = (dp[i - 1] + a[i - 2]).min(dp[i - 2] + b[i - 3]);
    }
    let mut root = vec![n];
    let mut step = n;
    while 1 < step {
        if dp[step] == dp[step - 1] + a[step - 2] {
            root.push(step - 1);
            step -= 1;
        } else {
            root.push(step - 2);
            step -= 2;
        }
    }
    root.reverse();
    println!("{}", root.len());
    for i in root {
        print!("{} ", i);
    }
}
