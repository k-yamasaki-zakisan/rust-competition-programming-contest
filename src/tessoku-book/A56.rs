use proconio::input;
use proconio::marker::Bytes;

// Bytesで文字列を範囲で取れる
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Bytes
    }
    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        if s[a - 1..b] == s[c - 1..d] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
