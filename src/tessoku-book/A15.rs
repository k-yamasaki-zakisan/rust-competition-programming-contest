use proconio::input;

use std::collections::HashMap;

// 座標圧縮
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut a_clone = a.clone();
    a_clone.sort();

    let mut nums_map = HashMap::new();
    let mut cnt = 0;
    for aa in a_clone {
        nums_map.entry(aa).or_insert_with(|| {
            cnt += 1;
            cnt
        });
    }
    let mut ans = vec![];
    for aa in a {
        ans.push(nums_map[&aa])
    }
    // 配列空白区切りで表示
    let dst: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
    println!("{}", dst.join(" "));
}
