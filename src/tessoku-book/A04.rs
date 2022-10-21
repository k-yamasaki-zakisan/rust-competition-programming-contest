use proconio::input;

// 10 -> 2進数変換
fn main() {
    input! {
      n: i32
    }

    let ans = format!("{:b}", n);
    println!("{}", ans)
}