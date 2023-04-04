// 標準入力から読み込みのためのライブラリをインポート
use std::io::{stdin, BufRead};

// ハッシュ関数に必要な定数
const BASE: u64 = 131;
const MOD: u64 = 1e9 as u64 + 7;

// ローリングハッシュ
fn main() {
    // 標準入力から1行ずつ読み込むための準備
    let stdin = stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    // 最初の行に書かれた整数を読み込む
    let nq: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = nq[0]; // 文字列の長さ
    let s: Vec<char> = lines.next().unwrap().chars().collect(); // 文字列
    let q = nq[1]; // クエリの数

    // ハッシュ値を計算するための準備
    let mut h: Vec<u64> = vec![0; n + 1];
    let mut p: Vec<u64> = vec![0; n + 1];
    let mut rev_h: Vec<u64> = vec![0; n + 1];
    let mut rev_p: Vec<u64> = vec![0; n + 1];
    p[0] = 1;
    rev_p[0] = 1;
    // ハッシュ値を計算する
    for i in 0..n {
        let c = s[i] as u64;
        h[i + 1] = (h[i] * BASE + c) % MOD; // 順方向のハッシュ値
        p[i + 1] = p[i] * BASE % MOD;
        let rc = s[n - i - 1] as u64;
        rev_h[i + 1] = (rev_h[i] * BASE + rc) % MOD; // 逆方向のハッシュ値
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
        let fh = (h[r] + MOD - h[l] * p[len] % MOD) % MOD; // 順方向のハッシュ値を計算
        let rev_fh = (rev_h[n - l] + MOD - rev_h[n - r] * rev_p[len] % MOD) % MOD; // 逆方向のハッシュ値を計算
        println!("{}", if fh == rev_fh { "Yes" } else { "No" });
    }
}
