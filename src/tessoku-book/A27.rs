use proconio::input;

// ユーグリッド除法（最大公約数）
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", gcd(a, b));
}

fn gcd(c: usize, d: usize) -> usize {
    if d == 0 {
        c
    } else {
        gcd(d, c % d)
    }
}
