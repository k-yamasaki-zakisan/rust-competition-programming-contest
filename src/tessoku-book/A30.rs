use proconio::input;
const MOD: i64 = 1000000007;

// pow mod
fn main() {
    input! {
        n: usize,
        r: usize,
    }
    let mut fact = vec![1; n + 1];
    let mut fact_inv = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i % (MOD as usize)
    }
    fact_inv[n] = pow((fact[n] as i64), MOD - 2);
    for i in (1..=n).rev() {
        fact_inv[i - 1] = ((i as i64) * fact_inv[i]) % MOD
    }
    let ans_1 = (((fact[n] as i64) * fact_inv[r] % MOD) * fact_inv[n - r]) % MOD;
    println!("{}", ans_1);
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

// fact = [1] * (N+1)
// fact_inv = [1] * (N+1)
// for i in range(1,N+1):
//     fact[i] = fact[i-1] * i % MOD
// fact_inv[N] = pow(fact[N],MOD-2,MOD)
// for i in range(N,0,-1):
//     fact_inv[i-1] = (i * fact_inv[i]) % MOD
// ### nCrの計算
// def comb(n,r):
//   return fact[n] * fact_inv[r] * fact_inv[n-r] % MOD
