use std::io::{stdin, BufRead};

const BASE: u64 = 131;
const MOD: u64 = 1e9 as u64 + 7;

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let nq: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = nq[0];
    let s: Vec<char> = lines.next().unwrap().chars().collect();
    let q = nq[1];
    let mut h: Vec<u64> = vec![0; n + 1];
    let mut p: Vec<u64> = vec![0; n + 1];
    let mut rev_h: Vec<u64> = vec![0; n + 1];
    let mut rev_p: Vec<u64> = vec![0; n + 1];
    p[0] = 1;
    rev_p[0] = 1;
    for i in 0..n {
        let c = s[i] as u64;
        h[i + 1] = (h[i] * BASE + c) % MOD;
        p[i + 1] = p[i] * BASE % MOD;
        let rc = s[n - i - 1] as u64;
        rev_h[i + 1] = (rev_h[i] * BASE + rc) % MOD;
        rev_p[i + 1] = rev_p[i] * BASE % MOD;
    }
    for _ in 0..q {
        let lr: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let l = lr[0] - 1;
        let r = lr[1];
        let len = r - l;
        let fh = (h[r] + MOD - h[l] * p[len] % MOD) % MOD;
        let rev_fh = (rev_h[n - l] + MOD - rev_h[n - r] * rev_p[len] % MOD) % MOD;
        if fh == rev_fh {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
