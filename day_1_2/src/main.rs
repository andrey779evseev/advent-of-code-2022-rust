fn main() {
    let mut elves = include_str!("input.txt")
        .split("\n\n")
        .map(|e| {
            e.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    elves.sort();
    elves.reverse();
    println!("{}", elves.iter().take(3).sum::<i32>());
}
