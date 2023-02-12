use proconio::input;

// 累積和
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize);q]
    }
    let mut ruseki = vec![];
    let mut cnt = 0;
    for a_val in a {
        cnt += a_val;
        ruseki.push(cnt);
    }
    for (l, r) in lr {
        let mut cnt_one = 0;
        if l == 1 {
            cnt_one += ruseki[r - 1];
        } else {
            cnt_one += ruseki[r - 1] - ruseki[l - 2];
        }

        let total_cnt = r - l + 1;
        if (total_cnt < cnt_one * 2) {
            println!("{}", "win");
        } else if (total_cnt == cnt_one * 2) {
            println!("{}", "draw");
        } else {
            println!("{}", "lose");
        }
    }
}
