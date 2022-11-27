use proconio::input;

// 集合
fn main() {
    input! {
        n: usize,
    }
    let cnt_3 = n / 3;
    let cnt_5 = n / 5;
    let cnt_3_5 = n / 15;
    println!("{}", (cnt_3 + cnt_5 - cnt_3_5))
}
