use proconio::input;

// よくわからん分類
fn main() {
    input! {
        n: usize,
        mut lr: [(i32, i32); n],
    }
    lr.sort_by(|l, r| l.1.partial_cmp(&(r.1)).unwrap());
    // println!("{:?}", lr);
    let mut now = 0;
    let mut cnt = 0;
    for (l, r) in lr {
        if now <= l {
            cnt += 1;
            now = r;
        }
    }
    println!("{}", cnt)
}
