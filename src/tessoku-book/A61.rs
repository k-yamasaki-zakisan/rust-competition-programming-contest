use proconio::input;

// vec -> 文字列変換（Pythonに比べてかなりめんどくさいな...）
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut root = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        root[a].push(b);
        root[b].push(a);
    }
    for i in 1..=n {
        // 配列の文字列化
        let children = root[i]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("{}: {{{}}}", i, children);
    }
}
