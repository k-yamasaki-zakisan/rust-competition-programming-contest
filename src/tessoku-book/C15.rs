use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::{
    cmp::min, cmp::Reverse, collections::BTreeSet, collections::BinaryHeap, collections::HashMap,
    collections::VecDeque,
};

const MOD: usize = 1000000007;
const MOD_I64: i64 = 1000000007;
const INF: usize = 1 << 62;
const INF_MINUS: i64 = -1000000007;

// 左右の累積和
fn main() {
    input! {
        n: usize,
        k: usize,
        lr: [(usize,usize); n],
    }
    let mut meeting = vec![];
    for (l, r) in lr {
        meeting.push((l, r + k));
    }
    let mut meeting_l = meeting.clone();
    meeting_l.sort_by(|l, r| l.1.partial_cmp(&(r.1)).unwrap());
    let max_t = meeting_l[n - 1].1;
    let mut cnt_l = vec![0; max_t + 1];
    let mut time = 0;
    let mut attended = 0;
    for (ms, mg) in meeting_l {
        if time <= ms {
            attended += 1;
            time = mg;
            cnt_l[time] = attended;
        }
    }
    let mut meeting_r = meeting.clone();
    meeting_r.sort();
    meeting_r.reverse();
    let t = meeting_r[n - 1].1;
    let mut cnt_r = vec![0; max_t + 1];
    let mut time = max_t;
    let mut attended = 0;
    for (ms, mg) in meeting_r {
        if mg <= time {
            attended += 1;
            time = ms;
            cnt_r[time] = attended;
        }
    }
    for i in 0..max_t {
        cnt_l[i + 1] = cnt_l[i].max(cnt_l[i + 1]);
        cnt_r[max_t - i - 1] = cnt_r[max_t - i - 1].max(cnt_r[max_t - i]);
    }
    for i in 0..n {
        println!("{}", cnt_l[meeting[i].0] + 1 + cnt_r[meeting[i].1])
    }
}
