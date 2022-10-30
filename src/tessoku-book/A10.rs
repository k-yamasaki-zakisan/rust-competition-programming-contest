use proconio::input;

// 右左のその地点のmax値をメモ
fn main() {
    input! {
        n: usize,
        a:[usize; n],
        d: usize,
        lr: [(usize, usize); d],
    }
    let mut l_max = vec![0; n + 1];
    let mut r_max = vec![0; n + 1];
    l_max[0] = a[0];
    for i in 1..n {
        l_max[i] = l_max[i - 1].max(a[i]);
    }
    r_max[n - 1] = a[n - 1];
    for i in (0..n - 1).rev() {
        r_max[i] = r_max[i + 1].max(a[i]);
    }

    for (l, r) in lr {
        let mut res = 0;
        if 1 < l {
            res = res.max(l_max[l - 2]);
        }
        if r < n {
            res = res.max(r_max[r]);
        }
        println!("{}", res);
    }
}
