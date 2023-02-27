use std::fs;

fn main() {
    let data = fs::read_to_string("day_1/src/data.txt").unwrap();
    let elves: Vec<&str> = data.split("\n\n").collect();
    let elves: Vec<i32> = elves
        .iter()
        .map(|elf| {
            let calories: Vec<&str> = elf.split_whitespace().collect();
            let calories: Vec<i32> = calories
                .iter()
                .map(|str_calorie| str_calorie.parse().unwrap())
                .collect();
            calories.iter().sum::<i32>()
        })
        .collect();

    part_one(&elves);

    part_two(elves);
}

fn part_one(elves: &Vec<i32>) {
    println!("part one: {}", elves.iter().max().unwrap());
}

fn part_two(mut elves: Vec<i32>) {
    let mut total = 0;
    for _ in 0..3 {
        let max = elves.iter().max().unwrap();
        total = total + max;
        if let Some(pos) = elves.iter().position(|x| *x == *max) {
            elves.remove(pos);
        }
    }
    println!("part two: {total}",);
}
