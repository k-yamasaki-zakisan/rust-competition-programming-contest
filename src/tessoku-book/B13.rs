use proconio::input;

// 尺取法
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut tmp: usize = 0;
    let mut head = 0;
    let mut ans: usize = 0;
    for i in 0..n {
        while head < n && tmp + a[head] <= k {
            tmp += a[head];
            head += 1
        }
        tmp -= a[i];
        ans += head - i;
    }
    println!("{}", ans);
}
