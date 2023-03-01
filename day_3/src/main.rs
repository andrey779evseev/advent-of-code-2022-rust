use std::fs;

fn main() {
    let data = fs::read_to_string("day_3/src/data.txt").unwrap();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let rucksacks = data.lines();
    let mut sum_of_priorities = 0;
    for rucksack in rucksacks {
        let mut first_compartment: Vec<String> = rucksack.chars().map(|c| c.to_string()).collect();
        let middle = first_compartment.len() / 2;
        let second_compartment: Vec<_> = first_compartment.splice(0..middle, []).collect();
        for item in first_compartment {
            if second_compartment.contains(&item) {
                let byte_item = item.as_bytes()[0];
                if byte_item >= b'a' {
                    sum_of_priorities += (byte_item - b'a') as i32 + 1;
                } else {
                    sum_of_priorities += (byte_item - b'A') as i32 + 27;
                }
                break;
            }
        }
    }
    println!("{sum_of_priorities}");
}

fn part_two(data: &str) {
    let rucksacks = data.lines();
    let rucksacks: Vec<&str> = rucksacks.collect();
    let groups: Vec<&[&str]> = rucksacks.chunks(3).collect();
    let mut sum_of_priorities = 0;
    for group in groups {
        let rucksacks: Vec<Vec<String>> = group
            .iter()
            .map(|r| r.chars().map(|c| c.to_string()).collect::<Vec<String>>())
            .collect();
        let badge = rucksacks[0]
            .iter()
            .find(|r| rucksacks[1].contains(r) && rucksacks[2].contains(r))
            .unwrap()
            .as_bytes()[0];
        if badge >= b'a' {
            sum_of_priorities += (badge - b'a') as i32 + 1;
        } else {
            sum_of_priorities += (badge - b'A') as i32 + 27;
        }
    }
    println!("{sum_of_priorities}");
}
