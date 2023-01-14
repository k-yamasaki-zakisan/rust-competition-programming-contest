use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut nums_map = HashMap::new();
    let mut cnt = 0;
    for aa in a {
        nums_map.entry(aa).or_insert_with(|| {
            cnt += 1;
            cnt
        });
    }

    let mut bisect_array: VecDeque<usize> = VecDeque::new();
    let mut ans = vec![-1];
    for aa in a {
        if bisect_array.len() == 0 {
            bisect_array.push_front(aa);
            continue;
        }
        let mut l = 0;
        let mut r = bisect_array.len();
        while 1 < r - l {
            let c = (l + r) / 2;
            if a < bisect_array[c] {
                l = c;
            } else {
                r = c;
            }
        }
    }
}
