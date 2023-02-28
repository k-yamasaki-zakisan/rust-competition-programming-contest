use proconio::input;

// 最小公倍数
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let gcd_num = gcd(a, b);
    let ans = a / gcd_num * b;
    println!("{}", ans)
}

fn gcd(c: usize, d: usize) -> usize {
    if d == 0 {
        c
    } else {
        gcd(d, c % d)
    }
}
