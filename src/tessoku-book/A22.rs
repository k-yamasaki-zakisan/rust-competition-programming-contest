use proconio::input;

// dp
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    }
    let inf = -1;
    let mut memo = vec![inf; n];
    memo[0] = 0;
    for i in 0..n - 1 {
        let (a_val, b_val) = (a[i] - 1, b[i] - 1);
        if memo[i] != inf {
            memo[a_val] = memo[a_val].max(memo[i] + 100);
            memo[b_val] = memo[b_val].max(memo[i] + 150);
        }
    }
    println!("{}", memo[n - 1]);
}
