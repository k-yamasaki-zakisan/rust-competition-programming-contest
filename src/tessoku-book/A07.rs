use proconio::input;

// 累積和
fn main() {
    input! {
        d:usize,
        n:usize,
        lr:[(usize, usize); n]
    }
    let mut ans = vec![0; d];
    for (l, r) in lr {
        ans[l - 1] += 1;
        if r < d {
            ans[r] -= 1;
        }
    }
    for i in 1..d {
        ans[i] += ans[i - 1];
    }
    for a in ans {
        println!("{}", a);
    }
}
