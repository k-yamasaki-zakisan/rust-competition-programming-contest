use proconio::input;

// ２次元累積和
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut cum = vec![vec![0; w]; h];
    for (a, b, c, d) in abcd.into_iter() {
        cum[a - 1][b - 1] += 1;
        if d < w && c < h {
            cum[c][d] += 1;
        }
        if d < w {
            cum[a - 1][d] -= 1;
        }
        if c < h {
            cum[c][b - 1] -= 1;
        }
    }
    for i in 0..h {
        for j in 1..w {
            cum[i][j] += cum[i][j - 1];
        }
    }
    for j in 0..w {
        for i in 1..h {
            cum[i][j] += cum[i - 1][j];
        }
    }
    // 2次元配列を１列ごとの空白文字列に変換して表示
    for i in 0..h {
        for j in 0..w {
            print!("{} ", cum[i][j]);
        }
        println!("");
    }
}
