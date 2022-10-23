use proconio::input;

// 累積和
fn main() {
    input! {
        n:i32,
        q:i32,
        a: [i32; n],
        lr:[(usize, usize); q]
    }

    let mut a_vec = vec![0];
    for a_num in a.iter() {
        a_vec.push(a_vec[a_vec.len() - 1] + a_num)
    }

    for (l, r) in lr {
        println!("{}", a_vec[r] - a_vec[l - 1])
    }
}
