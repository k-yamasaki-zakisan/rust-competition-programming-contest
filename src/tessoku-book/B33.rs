use proconio::input;

const MOD: usize = 1000000007;

// 2次元移動（先攻・後攻）
fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        ab:[(usize, usize); n],
    }
    let mut xor = 0;
    for i in 0..n {
        xor = (xor ^ (ab[i].0 - 1));
    }
    for i in 0..n {
        xor = (xor ^ (ab[i].1 - 1));
    }
    println!("{}", if xor != 0 { "First" } else { "Second" })
}
