use proconio::input;

// よくわからん分類
fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        a: [usize; n],
        c: [usize; m],
    }
    let mut ans = b * (n * m);
    for a_num in a {
        ans += a_num * m
    }
    for c_num in c {
        ans += c_num * n
    }
    println!("{}", ans)
}
