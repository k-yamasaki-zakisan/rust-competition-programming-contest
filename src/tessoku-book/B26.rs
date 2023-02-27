use proconio::input;

// エラトステネスの篩
fn main() {
    input! {
        n: usize,
    }
    let mut nums = vec![0; n + 10];
    for i in 2..=n {
        let mut num = i;
        while num <= n {
            nums[num] += 1;
            num += i;
        }
    }

    for i in 2..=n {
        if nums[i] == 1 {
            println!("{}", i);
        }
    }
}
