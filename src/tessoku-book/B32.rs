use proconio::input;

const MOD: usize = 1000000007;

// 石取り合戦（先攻・後攻）
fn main() {
    input! {
        n: usize,
        k: usize,
        a:[usize; k],
    }
    let mut state = vec![false; n + 1];
    for i in 0..n + 1 {
        for &a_val in &a {
            if a_val <= i && !state[i - a_val] {
                state[i] = true;
                break;
            }
        }
    }
    println!("{}", if state[n] { "First" } else { "Second" });
}
