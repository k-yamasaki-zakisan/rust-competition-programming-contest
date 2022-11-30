use proconio::input;

// xor
fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }
    let mut ans = 0;
    for cc in c.iter().take(n) {
        ans ^= cc
    }
    println!("{}", if ans == 0 { "Second" } else { "First" });
}
