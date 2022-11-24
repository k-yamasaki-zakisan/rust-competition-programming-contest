use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(char, i32); n],
    }
    let mod_num = 10000;
    let mut num = 0;
    for (t, a) in ta {
        if t == '+' {
            num += a;
        }
        if t == '-' {
            num -= a;
            if num < 0 {
                num += mod_num;
            }
        }
        if t == '*' {
            num *= a;
        }
        num %= mod_num;
        println!("{}", num);
    }
}
