use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const INF: usize = 1 << 62;

// 0 ~ 9999の全探索
fn main() {
    input! {
        n: usize,
        st: [(Chars, usize); n],
    }
    let mut ans = vec![];
    for num in 0..=9999 {
        let num_str = zfill(&num.to_string(), 4);
        let num_chars: Vec<char> = num_str.chars().collect();
        let mut is_ok = true;
        for (s, t) in st.clone() {
            let mut diff_cnt = 0;
            for i in 0..4 {
                if num_chars[i] != s[i] {
                    diff_cnt += 1;
                }
            }
            if t == 1 && diff_cnt != 0
                || t == 2 && diff_cnt != 1
                || t == 3 && diff_cnt == 0
                || t == 3 && diff_cnt == 1
            {
                is_ok = false;
            }
        }
        if is_ok {
            ans.push(num_str);
        }
    }
    println!(
        "{}",
        if ans.len() == 1 {
            ans[0].clone()
        } else {
            "Can't Solve".to_string()
        }
    )
}

fn zfill(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s.to_string();
    }
    let zeros = "0".repeat(width - s.len());
    return zeros + s;
}
