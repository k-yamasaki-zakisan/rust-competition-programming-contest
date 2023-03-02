use proconio::input;

const MOD: usize = 1000000007;

// ベン図
fn main() {
    input! {
        n: usize,
    }
    let mod_3 = n / 3;
    let mod_5 = n / 5;
    let mod_7 = n / 7;
    let mod_15 = n / 15;
    let mod_21 = n / 21;
    let mod_35 = n / 35;
    let mod_105 = n / 105;
    let ans = mod_3 + mod_5 + mod_7 - mod_15 - mod_21 - mod_35 + mod_105;
    println!("{}", ans);
}
