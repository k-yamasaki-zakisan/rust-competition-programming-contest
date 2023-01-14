use proconio::input;

// 配列内をいい感じに処理する
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut stoke: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        while !stoke.is_empty() {
            let (mut k, mut v) = stoke.last().unwrap();
            if a[i] < v {
                print!("{}", k + 1);
                break;
            } else {
                stoke.pop();
            }
        }
        if stoke.is_empty() {
            print!("-1");
        }
        stoke.push((i, a[i]));
        if i != n - 1 {
            print!(" ")
        }
    }
    println!("");
}
