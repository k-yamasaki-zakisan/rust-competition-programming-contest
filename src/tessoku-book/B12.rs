use proconio::input;

// 二分探索
fn main() {
    input! {
        n: f64,
    }
    let mut l = 0.00;
    let mut r: f64 = n;
    while 1.0 < r * 100000.0 - l * 100000.0 {
        let a = (l + r) / 2.0;
        if a * a * a + a <= n {
            l = a;
        } else {
            r = a;
        }
    }
    println!("{}", l)
}
