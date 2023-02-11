use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let target = 1000;
    let mut flag = false;
    for i in 0..n - 2 {
        let a_i = a[i];
        for j in i + 1..n - 1 {
            let a_j = a[j];
            for k in j + 1..n {
                if a_i + a_j + a[k] == target {
                    flag = true;
                    break;
                }
            }
            if flag {
                break;
            }
        }
        if flag {
            break;
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}
