use std::io;

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).ok();
    let n_int:i32 = n_str.trim().parse().ok().unwrap();

    println!("{}", n_int*n_int)
}