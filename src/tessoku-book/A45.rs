use proconio::input;
use proconio::marker::Chars;

// 反転管理
fn main() {
    input! {
        n: usize,
        c: Chars,
        a: Chars,
    }
    let mut ans = 0;
    for aa in a {
        if aa == 'R' {
            ans += 1;
        }
        if aa == 'B' {
            ans += 2;
        }
        if aa == 'W' {
            ans += 0;
        }
        ans %= 3;
    }
    if c[0] == 'W' && ans == 0 || c[0] == 'R' && ans == 1 || c[0] == 'B' && ans == 2 {
        println!("Yes");
    } else {
        println!("No")
    }
}
