use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1000000007;

// 山の問題（左右からリニアスキャンして見る配列を作るだけ）
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut left = vec![0; n];
    left[0] = 1;
    let mut now = 1;
    for i in 0..n - 1 {
        if s[i] == 'A' {
            now += 1;
        } else {
            now = 1;
        }
        left[i + 1] = now;
    }
    let mut right = vec![0; n];
    right[n - 1] = 1;
    for i in (0..n - 1).rev() {
        if s[i] == 'B' {
            now += 1;
        } else {
            now = 1;
        }
        right[i] = now;
    }
    // println!("{:?}", left);
    // println!("{:?}", right);
    let mut ans = 0;
    for i in 0..n {
        ans += left[i].max(right[i]);
    }
    println!("{}", ans);
}
