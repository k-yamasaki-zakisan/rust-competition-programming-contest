use proconio::input;

// 累積和
fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut ans = vec![0; t + 10];
    for &(l, r) in &lr {
        ans[l] += 1;
        ans[r] -= 1;
    }
    for i in 1..=t {
        ans[i] += ans[i - 1];
    }
    for a_val in ans.iter().take(t) {
        println!("{}", a_val);
    }
}
