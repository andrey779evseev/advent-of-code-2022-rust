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

        for _ in 0..count {
            let el = stacks[from].pop().unwrap();
            stacks[to].push(el);
        }
    });

    println!(
        "{}",
        stacks
            .map(|s| s.last().unwrap().to_owned())
            .iter()
            .collect::<String>()
    );
}
