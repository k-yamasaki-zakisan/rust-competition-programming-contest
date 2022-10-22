use proconio::input;

// 10 -> 2進数変換
// 0埋め(Python: zfill)
fn main() {
    input! {
      n: i32
    }

    let ans = format!("{:b}", n);
    println!("{:0>10}", ans)
}