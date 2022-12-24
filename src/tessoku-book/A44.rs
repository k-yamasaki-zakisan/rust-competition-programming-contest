use proconio::input;

// 反転管理
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        ans[i] = i + 1
    }

    let mut is_rev = false;
    for _ in 0..q {
        input! {
            q_one: usize,
        };

        if q_one == 1 {
            input! {
                q_two: usize,
                q_three: usize,
            };
            if is_rev {
                ans[n - q_two] = q_three;
            } else {
                ans[q_two - 1] = q_three;
            }
        } else if q_one == 2 {
            is_rev = !is_rev;
        } else if q_one == 3 {
            input! {
                q_two: usize,
            };
            println!(
                "{}",
                if is_rev {
                    ans[n - q_two]
                } else {
                    ans[q_two - 1]
                }
            )
        };
    }
}
