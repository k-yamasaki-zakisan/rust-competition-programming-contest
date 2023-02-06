use proconio::input;

// 転倒回数？（バブルソート）
fn main() {
    input! {
        n: usize,
        mut p: [[usize; n]; n],
    }
    let mut rows = vec![0; n];
    let mut columns = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if p[i][j] != 0 {
                rows[i] = p[i][j];
                columns[j] = p[i][j];
            }
        }
    }
    let mut ans = 0;
    loop {
        let mut update = false;
        for i in 1..n {
            if rows[i - 1] > rows[i] {
                let swap = rows[i - 1];
                rows[i - 1] = rows[i];
                rows[i] = swap;
                ans += 1;
                update = true;
            }
        }

        if !update {
            break;
        }
    }
    loop {
        let mut update = false;
        for i in 1..n {
            if columns[i - 1] > columns[i] {
                let swap = columns[i - 1];
                columns[i - 1] = columns[i];
                columns[i] = swap;
                ans += 1;
                update = true;
            }
        }

        if !update {
            break;
        }
    }

    println!("{}", ans);
}
