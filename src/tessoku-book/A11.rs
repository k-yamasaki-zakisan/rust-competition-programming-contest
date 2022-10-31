use proconio::input;

// ２分探索
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let i = bis(a.len() as i64, -1, |i| a[i as usize] > x);
    println!("{}", i)
}

fn bis<P>(ok: i64, ng: i64, p: P) -> i64
where
    P: Fn(i64) -> bool,
{
    let mid = (ok + ng) / 2;
    if (ok - ng).abs() == 1 {
        ok
    } else if p(mid) {
        bis(mid, ng, p)
    } else {
        bis(ok, mid, p)
    }
}
