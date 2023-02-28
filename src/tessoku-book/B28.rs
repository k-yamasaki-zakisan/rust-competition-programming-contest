use proconio::input;

const MOD: usize = 1000000007;

// フィボナッチ（mod）
fn main() {
    input! {
        n: usize,
    }
    let mut a_1 = 1;
    let mut a_2 = 1;
    let mut a_3 = 2;
    for _ in 0..n - 3 {
        a_1 = a_2;
        let tmp = a_3;
        a_3 = a_3 + a_2;
        a_2 = tmp;

        a_3 %= MOD;
    }
    println!("{}", a_3);
}
