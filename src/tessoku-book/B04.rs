use proconio::input;
use proconio::marker::Chars;

// 2進数 -> 10進数変換
fn main() {
    input! {
        n: Chars,
    }
    let mut ans = 0;
    for (i, &c) in n.iter().rev().enumerate() {
        if c == '1' {
            ans += 1 << i;
        }
    }
    println!("{}", ans);
}
