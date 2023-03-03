fn main() {
    let data = include_str!("input.txt");
    let (s, o) = data.split_once("\n\n").unwrap();
    let mut stacks: [Vec<char>; 9] = Default::default();

    s.lines().rev().skip(1).for_each(|l| {
        l.chars().skip(1).step_by(4).enumerate().for_each(|(i, x)| {
            if x != ' ' {
                stacks[i].push(x)
            }
        })
    });

    o.split('\n').for_each(|o| {
        let splitted: Vec<_> = o.split_whitespace().collect();
        let count = splitted[1].parse::<usize>().unwrap();
        let from = splitted[3].parse::<usize>().unwrap() - 1;
        let to = splitted[5].parse::<usize>().unwrap() - 1;

        let final_length = stacks[from].len() - count;
        let mut elements = stacks[from].split_off(final_length);
        stacks[to].append(&mut elements);
    });

    println!(
        "{}",
        stacks
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.last().unwrap().to_owned())
            .collect::<String>()
    );
}
