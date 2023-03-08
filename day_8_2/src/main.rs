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
                    .map(|x| {
                        let (mut top, mut bottom, mut right, mut left) = (1, 1, 1, 1);
                        for lookup_y in (1..y).rev() {
                            if trees[lookup_y][x] >= trees[y][x] {
                                break;
                            } else {
                                top += 1;
                            }
                        }
                        for lookup_y in (y + 1)..size {
                            if trees[lookup_y][x] >= trees[y][x] {
                                break;
                            } else if trees.len() - 1 != lookup_y {
                                bottom += 1;
                            }
                        }
                        for lookup_x in (1..x).rev() {
                            if trees[y][lookup_x] >= trees[y][x] {
                                break;
                            } else {
                                left += 1;
                            }
                        }
                        for lookup_x in (x + 1)..size {
                            if trees[y][lookup_x] >= trees[y][x] {
                                break;
                            } else if trees[y].len() - 1 != lookup_x {
                                right += 1;
                            }
                        }
                        top * bottom * left * right
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .max()
            .unwrap()
    );
}
