use proconio::input;

// xor
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }
    let max_num = 100000 + 1;
    let mut g = vec![0; max_num];
    for i in 0..max_num {
        let mut c = [true; 3];
        if x <= i {
            c[g[i - x]] = false;
        }
        if y <= i {
            c[g[i - y]] = false
        }
        for j in 0..3 {
            if c[j] {
                g[i] = j;
                break;
            }
        }
    }
    let mut ans = g[a[0]];
    for i in 1..n {
        ans ^= g[a[i]]
    }
    println!("{}", if ans == 0 { "Second" } else { "First" });
}
