use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1000000007;

//
fn main() {
    input! {
        n: i64,
    }
    let mut num = n;
    let mut ans = 0;
    let mut p = 1;
    let mut q = 0;
    while 0 < num {
        let mod_val = num % 10;
        num = num / 10;
        ans += num * 45 * p;
        ans += (mod_val - 1) * mod_val / 2 * p;
        ans += mod_val;
        ans += mod_val * q;
        q += mod_val * p;
        p *= 10;
    }
    println!("{}", ans);
}
