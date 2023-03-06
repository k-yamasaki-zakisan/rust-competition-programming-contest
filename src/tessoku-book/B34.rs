use proconio::input;

const MOD: usize = 1000000007;

// 配列中の石取りゲーム（先攻・後攻）
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a:[usize; n],
    }
    let mut xor = 0;
    for &a_val in &a {
        if (a_val % 5 == 2 || a_val % 5 == 3) {
            xor ^= 1
        } else if (a_val % 5 == 4) {
            xor ^= 2;
        }
    }
    println!("{}", if xor != 0 { "First" } else { "Second" })
}
