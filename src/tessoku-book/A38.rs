use proconio::input;

// よくわからん分類
fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n],
    }
    let max_num = 24;
    let mut hours = vec![max_num; d + 1];
    hours[0] = 0;
    for (l, r, h) in lrh {
        for i in l..=r {
            hours[i] = hours[i].min(h)
        }
    }
    let mut ans = 0;
    for hour in hours {
        ans += hour;
    }
    println!("{}", ans)
}
