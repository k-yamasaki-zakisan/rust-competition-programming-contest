use proconio::input;

// 貪欲
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }
    a.sort();
    b.sort();
    b.reverse();

    let mut ans = 0;
    for i in 0..n {
        ans += a[i] * b[i];
    }
    println!("{}", ans);
}
