fn main() {
    let x_int = 2.655 as i32;
    println!("{}", x_int);
}

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; n.try_into().unwrap()]; n.try_into().unwrap()];
        for query in queries {
            for h in query[0]..query[2] + 1 {
                for w in query[1]..query[3] + 1 {
                    ans[h][w] += 1
                }
            }
        }
        ans
    }
}
