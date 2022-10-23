use proconio::input;

fn main() {
    input! {
      n: i32,
      x: i32,
      a: [i32; n],
    }
    for num in a {
        if num == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
