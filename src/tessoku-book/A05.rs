use proconio::input;

// ２重ループ全探索
fn main() {
    input! {
      nk: [i32; 2],
    }

    let n = nk[0];
    let k = nk[1];
    let mut ans = 0;
    for i in 1..n+1 {
      for j in 1..n+1 {
        let tmp = k-(i+j);
        if  tmp < 1 {
          break
        } else if tmp <= n {
          ans += 1
        }
      }
    }
    println!("{}", ans);
}