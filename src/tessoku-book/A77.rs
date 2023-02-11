use proconio::input;

// 値決め打ち２分探索
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.push(l);
    let mut diffs = vec![a[0]];
    for i in 1..a.len() {
        diffs.push(a[i] - a[i - 1]);
    }

    let mut l_num = 0;
    let mut r_num = 1000000000;
    while 1 < r_num - l_num {
        let base = (l_num + r_num) / 2;
        let mut peaces = vec![];
        let mut tmp_peace = 0;
        let mut cnt = 0;
        for &diff in &diffs {
            tmp_peace += diff;
            if base < tmp_peace {
                cnt += 1;
                peaces.push(tmp_peace);
                tmp_peace = 0;
            }
        }

        if k < cnt {
            l_num = base;
        } else {
            r_num = base;
        }
    }
    println!("{}", r_num);
}
