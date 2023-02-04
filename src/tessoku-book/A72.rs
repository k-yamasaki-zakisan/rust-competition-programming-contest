use proconio::input;
use proconio::marker::Chars;

// 横のbit全探索、縦の貪欲（sort）
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        a: [Chars; h],
    }

    let mut ans = 0;
    for i in 0..(1 << h) {
        let mut h_choices = vec![];
        for j in 0..h {
            if (1 << j) & i == 0 {
                h_choices.push(j)
            }
        }
        if k < h_choices.len() {
            continue;
        }
        let mut cnt: usize = h_choices.len() * w;
        let mut w_cnts = vec![];
        for w_i in 0..w {
            let mut tmp = 0;
            for h_i in 0..h {
                if h_choices.contains(&h_i) {
                    continue;
                }
                if a[h_i][w_i] == '#' {
                    tmp += 1;
                }
            }
            w_cnts.push(tmp);
        }
        w_cnts.sort();
        for i in 0..(k - h_choices.len()) {
            cnt += h - h_choices.len();
        }
        for i in (k - h_choices.len())..w {
            cnt += w_cnts[i];
        }
        ans = ans.max(cnt);
    }
    println!("{}", ans);
}
