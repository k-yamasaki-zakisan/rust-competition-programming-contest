use proconio::input;

// stack
fn main() {
    input! {
        q: usize,
    }
    let mut memo: Vec<String> = vec![];
    for _ in 0..q {
        input! {
            x: usize
        }
        if x == 1 {
            input! {
                y: String
            }
            memo.push(y);
        } else if x == 2 {
            println!("{}", memo[memo.len() - 1]);
        } else {
            memo.pop();
        }
    }
}
