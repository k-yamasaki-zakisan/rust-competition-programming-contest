use proconio::input;

// エラトステネスの篩
fn main() {
    input! {
        q: usize,
        c: [usize; q],
    }
    let max_num = 300000;
    let mut memo = vec![0; max_num + 1];
    for i in 2..=max_num {
        let mut num = i;
        while num <= max_num {
            memo[num] += 1;
            num += i;
        }
    }
    for cc in c {
        if memo[cc] == 1 {
            println!("Yes");
        } else {
            println!("No")
        }
    }
}
