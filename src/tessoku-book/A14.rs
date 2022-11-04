use proconio::input;

use std::collections::HashSet;

// 二組に分けて検索
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }
    let mut ab = HashSet::new();
    for aa in a.iter().take(n) {
        for bb in b.iter().take(n) {
            ab.insert(aa + bb);
        }
    }
    let mut cd = HashSet::new();
    for cc in c.iter().take(n) {
        for dd in d.iter().take(n) {
            cd.insert(cc + dd);
        }
    }

    for abab in ab {
        let tmp = k - abab;
        if cd.contains(&tmp) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
