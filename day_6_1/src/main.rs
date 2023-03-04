fn main() {
    let chars: Vec<_> = include_str!("input.txt").chars().collect();
    println!(
        "{}",
        chars
            .iter()
            .enumerate()
            .find(|(i, x)| {
                if *i < 3 {
                    return false;
                }
                let mut seq = vec![*x, &chars[i - 1], &chars[i - 2], &chars[i - 3]];
                seq.sort();
                seq.dedup();
                seq.len() == 4
            })
            .unwrap()
            .0
            + 1
    );
}
