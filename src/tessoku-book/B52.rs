use proconio::input;
use proconio::marker::Chars;
use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap, collections::VecDeque};

const MOD: usize = 1000000007;

//
fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: Chars,
    }
    let mut stack = VecDeque::new();
    stack.push_back(x - 1);
    while 0 < stack.len() {
        let now = stack.pop_front().unwrap();
        a[now] = '@';
        if 0 < now - 1 && a[now - 1] == '.' {
            stack.push_back(now - 1)
        }
        if now + 1 < n && a[now + 1] == '.' {
            stack.push_back(now + 1)
        }
    }
    if 1 < n && a[0] == '.' && a[1] == '@' {
        a[0] = '@';
    }
    // 配列の文字列化
    let ans = a
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ans)
}
