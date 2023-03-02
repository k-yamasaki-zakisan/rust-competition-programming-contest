use proconio::input;

const MOD: usize = 1000000007;

// mod（２次元平面を右と下のみ移動で右下行く方法は何通り）
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    //  手順 1：分子を求める
    let mut a = 1;
    for i in 1..=(h + w - 2) {
        a = (a * i) % MOD;
    }
    // 手順 2：分母を求める
    let mut b = 1;
    for i in 1..h {
        b = (b * i) % MOD;
    }
    for i in 1..w {
        b = (b * i) % MOD;
    }
    //手順 3：答えを求める
    let ans = (a * mod_pow(b, MOD - 2, MOD)) % MOD;
    println!("{}", ans);
}

fn mod_pow(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut i = 0;
    let mut ans = 1;
    while 1 << i <= b {
        if b >> i & 1 == 1 {
            ans = (ans * p) % m;
        }
        p = (p * p) % m;
        i += 1
    }
    ans
}
