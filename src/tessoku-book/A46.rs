use proconio::input;

// 貪欲法
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    println!("1");

    let mut now = 0;
    let mut visited = vec![false; n];
    visited[0] = true;
    for _ in 0..n - 1 {
        let (nowx, nowy) = xy[now];
        let mut tmp = 10000000000;
        let mut tmp_position = now;
        for i in 0..n {
            if visited[i] {
                continue;
            }
            let (nextx, nexty) = xy[i];
            let mut diff = 0;
            // println!("{}-{}", i, now);
            if nextx != nowx {
                if nextx < nowx {
                    diff += usize::pow((nowx - nextx), 2);
                } else {
                    diff += usize::pow((nextx - nowx), 2);
                }
            }
            if nexty != nowy {
                if nexty < nowy {
                    diff += usize::pow(nowy - nexty, 2);
                } else {
                    diff += usize::pow(nexty - nowy, 2);
                }
            }

            if diff < tmp {
                tmp = diff;
                tmp_position = i;
            }
        }
        now = tmp_position;
        visited[now] = true;
        println!("{}", now + 1);
    }
    println!("1");
}
