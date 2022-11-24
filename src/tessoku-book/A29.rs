use proconio::input;
const MOD: i64 = 1000000007;

// pow mod
fn main() {
    input! {
        mut a: i64,
        mut b: i64,
    }
    println!("{}", pow(a, b));
}

fn pow(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    if b % 2 == 0 {
        pow(a * a % MOD, b / 2)
    } else {
        a * pow(a, b - 1) % MOD
    }
}
