use proconio::input;

// 二分探索
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    }

    a.sort();
    for &x_val in x.iter() {
        if x_val <= a[0] {
            println!("0");
            continue;
        }
        let mut l = 0;
        let mut r = a.len();
        while 1 < r - l {
            let ii = (l + r) / 2;
            if a[ii] < x_val {
                l = ii;
            } else {
                r = ii;
            }
        }
        println!("{}", r);
    }
}
