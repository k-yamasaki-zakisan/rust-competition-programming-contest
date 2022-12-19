use proconio::input;

// 3重ループで雑に全探索（参照権の使い方）
fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n]
    }
    let mut ans = 0;
    for tough in 0..100 {
        for willpower in 0..100 {
            let mut tmp = 0;
            for (a, b) in &ab {
                if tough <= *a && *a <= tough + k && willpower <= *b && *b <= willpower + k {
                    tmp += 1
                }
            }
            ans = ans.max(tmp);
        }
    }
    println!("{}", ans)
}
