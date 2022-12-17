use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (_, value) in map {
        if value >= 3 {
            ans += value * (value - 1) * (value - 2) / 6
        }
    }
    println!("{}", ans);
}
