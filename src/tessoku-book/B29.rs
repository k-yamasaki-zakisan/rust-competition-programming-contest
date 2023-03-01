use proconio::input;

const MOD: u64 = 1000000007;

// pow mod
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    println!("{}", mod_pow(a, b, MOD))
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}
