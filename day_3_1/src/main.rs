fn main() {
    println!(
        "{}",
        include_bytes!("input.txt")
            .split(|b| *b == b'\n')
            .map(|l| l.split_at(l.len() / 2))
            .map(|(a, b)| a
                .iter()
                .filter(|c| b.contains(c))
                .map(|i| {
                    if *i >= b'a' {
                        (i - b'a') as i32 + 1
                    } else {
                        (i - b'A') as i32 + 27
                    }
                })
                .collect::<Vec<i32>>()
                .first()
                .unwrap()
                .to_owned())
            .sum::<i32>()
    );
}
