fn main() {
    let chars: Vec<_> = include_str!("input.txt").chars().collect();
    println!(
        "{}",
        chars
            .iter()
            .enumerate()
            .find(|(i, x)| {
                if *i < 13 {
                    return false;
                }
                let mut seq = vec![*x];
                for j in 0..14 {
                    seq.push(&chars[i - j]);
                }
                seq.sort();
                seq.dedup();
                seq.len() == 14
            })
            .unwrap()
            .0
            + 1
    );
}
