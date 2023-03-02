fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|e| e
                .split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .sum::<i32>())
            .max()
            .unwrap()
    );
}
