use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        ab: [(usize, char); n]
    }
    let mut ans = 0;
    for &(a, b) in ab.iter() {
        ans = ans.max(if b == 'E' { l - a } else { a })
    }
    println!("{}", ans);
}
