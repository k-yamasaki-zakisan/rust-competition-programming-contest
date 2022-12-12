use proconio::input;

// よくわからん分類
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    if k < (n - 1) * 2 {
        println!("No");
    } else {
        let result = if (k - (n - 1) * 2) % 2 == 0 {
            "Yes"
        } else {
            "No"
        };
        println!("{}", result)
    }
}
