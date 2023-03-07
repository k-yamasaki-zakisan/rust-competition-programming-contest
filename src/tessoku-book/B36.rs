use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1000000007;

//
fn main() {
    input! {
        n: usize,
        k: i32,
        s: Chars,
    }
    let mut cnt = 0;
    for &s_val in &s {
        if s_val == '1' {
            cnt += 1;
        }
    }
    println!(
        "{}",
        if (cnt - k).abs() % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    )
}
