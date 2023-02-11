use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let mut flag = false;
    for num in a..=b {
        if 100 % num == 0 {
            flag = true;
            break;
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}
