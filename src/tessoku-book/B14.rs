use proconio::input;
use std::collections::HashSet;

// 半分全列挙
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    if n <= 15 {
        let mut a_all = HashSet::new();
        a_all.insert(0);
        for i in 0..n {
            let mut tmp_all = a_all.clone();
            for &val in &a_all {
                tmp_all.insert(val + a[i]);
            }
            a_all = tmp_all;
        }
        if a_all.contains(&k) {
            println!("Yes");
            return;
        }
    }

    let mut a_f_half = HashSet::new();
    a_f_half.insert(0);
    for i in 0..n / 2 {
        let mut tmp_f_half = a_f_half.clone();
        for &val in &a_f_half {
            tmp_f_half.insert(val + a[i]);
        }
        a_f_half = tmp_f_half;
    }

    let mut a_s_half = HashSet::new();
    a_s_half.insert(0);
    for i in n / 2..n {
        let mut tmp_s_half = a_s_half.clone();
        for &val in &a_f_half {
            tmp_s_half.insert(val + a[i]);
        }
        a_s_half = tmp_s_half;
    }

    for &val in &a_f_half {
        if a_s_half.contains(&(k - val)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
