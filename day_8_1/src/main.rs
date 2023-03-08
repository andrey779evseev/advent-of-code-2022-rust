fn main() {
    let trees = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let size = trees.len();
    println!(
        "{}",
        (0..size)
            .map(|y| {
                (0..size)
                    .filter(|x| {
                        let x = *x;
                        (0..y).all(|lookup_y| trees[lookup_y][x] < trees[y][x])
                            || ((y + 1)..size).all(|lookup_y| trees[lookup_y][x] < trees[y][x])
                            || (0..x).all(|lookup_x| trees[y][lookup_x] < trees[y][x])
                            || ((x + 1)..size).all(|lookup_x| trees[y][lookup_x] < trees[y][x])
                    })
                    .collect::<Vec<_>>()
                    .len()
            })
            .sum::<usize>()
    );
}
