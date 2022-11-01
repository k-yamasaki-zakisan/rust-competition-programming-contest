use proconio::input;

// ２分探索
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut l = 0;
    let mut r = 100000000000;
    while 1 < r - l {
        let c = (l + r) / 2;
        let mut tmp = 0;
        for aa in a.iter().take(n) {
            tmp += c / aa
        }
        if tmp < k {
            l = c;
        } else {
            r = c;
        }
    }
    let mut tmp_2 = 0;
    for aa in a.iter().take(n) {
        tmp_2 += l / aa
    }
    let ans = if k <= tmp_2 { l } else { r };
    println!("{}", ans)
}
