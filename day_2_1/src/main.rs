fn main() {
    println!(
        "{}",
        include_bytes!("input.txt")
            .split(|b| *b == b'\n')
            .map(|v| {
                match (v[0], v[2]) {
                    (b'A', b'X') => 3 + 1,
                    (b'A', b'Y') => 6 + 2,
                    (b'A', _) => 0 + 3,
                    (b'B', b'X') => 0 + 1,
                    (b'B', b'Y') => 3 + 2,
                    (b'B', _) => 6 + 3,
                    (_, b'X') => 6 + 1,
                    (_, b'Y') => 0 + 2,
                    (_, _) => 3 + 3,
                }
            })
            .sum::<i32>()
    );
}
