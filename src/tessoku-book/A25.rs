use proconio::input;
use proconio::marker::Chars;

// dp
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut dp = vec![vec![0 as i64; w]; h];
    dp[0][0] = 1;
    for hh in 0..h {
        for ww in 0..w {
            if c[hh][ww] == '#' {
                continue;
            }
            if 0 < hh {
                dp[hh][ww] += dp[hh - 1][ww];
            }
            if 0 < ww {
                dp[hh][ww] += dp[hh][ww - 1];
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
