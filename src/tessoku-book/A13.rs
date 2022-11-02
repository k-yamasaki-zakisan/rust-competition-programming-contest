use proconio::input;

// ２分探索
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        let target = a[i] + k;
        let mut l = 0;
        let mut r = a.len();
        while 1 < r - l {
            let ii = (l + r) / 2;
            if a[ii] <= target {
                l = ii;
            } else {
                r = ii;
            }
        }
        ans += l - i;
    }
    println!("{}", ans)
}
