fn main() {
    println!(
        "{}",
        include_bytes!("input.txt")
            .split(|b| *b == b'\n')
            .map(|v| {
                match (v[0], v[2]) {
                    (b'A', b'X') => 3 + 0,
                    (b'A', b'Y') => 1 + 3,
                    (b'A', _) => 2 + 6,
                    (b'B', b'X') => 1 + 0,
                    (b'B', b'Y') => 2 + 3,
                    (b'B', _) => 3 + 6,
                    (_, b'X') => 2 + 0,
                    (_, b'Y') => 3 + 3,
                    (_, _) => 1 + 6,
                }
            })
            .sum::<i32>()
    );
}
