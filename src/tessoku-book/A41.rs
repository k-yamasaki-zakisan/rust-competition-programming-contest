use proconio::input;
use proconio::marker::Chars;

// ３つの文字が同じかどうか
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut flag = false;
    for i in 1..n - 1 {
        if s[i - 1] == s[i] && s[i] == s[i + 1] {
            flag = true;
            break;
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}
