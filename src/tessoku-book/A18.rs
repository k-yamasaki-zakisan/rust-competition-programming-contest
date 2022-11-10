use proconio::input;

// dp easy
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![false; s + 1];
    dp[0] = true;
    for aa in a {
        let mut i = s;
        loop {
            if dp[i] && i + aa <= s {
                dp[i + aa] = true;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    // 三項演算子
    println!("{}", if dp[s] { "Yes" } else { "No" });
}
