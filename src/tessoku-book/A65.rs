use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
    }
    let mut cnts = vec![0; n];
    for i in (0..n - 1).rev() {
        cnts[a[i] - 1] += cnts[i + 1] + 1;
    }

    // 配列の文字列化
    let ans = cnts
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
