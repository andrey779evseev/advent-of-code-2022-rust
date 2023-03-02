fn main() {
    println!(
        "{}",
        include_bytes!("input.txt")
            .split(|b| *b == b'\n')
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|g| {
                g[0].iter()
                    .find(|i| g[1].contains(i) && g[2].contains(i))
                    .unwrap()
            })
            .map(|b| {
                if *b >= b'a' {
                    (b - b'a') as i32 + 1
                } else {
                    (b - b'A') as i32 + 27
                }
            })
            .sum::<i32>()
    );
}
