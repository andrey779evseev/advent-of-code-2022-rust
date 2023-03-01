use std::fs;

fn main() {
    let data = fs::read_to_string("day_2/src/data.txt").unwrap();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &String) {
    let mut score = 0;
    let rounds = data.lines();
    for round in rounds {
        let values: Vec<&str> = round.split_whitespace().collect();

        //1 for Rock, 2 for Paper, and 3 for Scissors
        //X for Rock, Y for Paper, and Z for Scissors
        //1 for X, 2 for Y, 3 for Z
        let first_player = values[0];
        let second_player = values[1];
        match second_player {
            "X" => score += 1,
            "Y" => score += 2,
            _ => score += 3,
        }
        match first_player {
            "A" => match second_player {
                "X" => score += 3,
                "Y" => score += 6,
                _ => {}
            },
            "B" => match second_player {
                "X" => {}
                "Y" => score += 3,
                _ => score += 6,
            },
            _ => match second_player {
                "X" => score += 6,
                "Y" => {}
                _ => score += 3,
            },
        }
    }
    println!("{score}");
}

fn part_two(data: &String) {
    let mut score = 0;
    let rounds = data.lines();
    for round in rounds {
        let values: Vec<&str> = round.split_whitespace().collect();
        //A for Rock, B for Paper, and C for Scissors
        //X for Rock, Y for Paper, and Z for Scissors
        //1 for X, 2 for Y, 3 for Z
        //X - lose, Y - draw, Z - win
        let first_player = values[0];
        let expected_result = values[1];
        match first_player {
            "A" => match expected_result {
                "X" => {
                    //for Scissors
                    score += 3;
                }
                "Y" => {
                    //for rock
                    score += 1;
                    //for draw
                    score += 3;
                }
                _ => {
                    //for paper
                    score += 2;
                    //for win
                    score += 6;
                }
            },
            "B" => match expected_result {
                "X" => {
                    //for rock
                    score += 1;
                }
                "Y" => {
                    //for paper
                    score += 2;
                    //for draw
                    score += 3;
                }
                _ => {
                    //for scissors
                    score += 3;
                    //for win
                    score += 6;
                }
            },
            _ => match expected_result {
                "X" => {
                    //for paper
                    score += 2;
                }
                "Y" => {
                    //for scissors
                    score += 3;
                    //for draw
                    score += 3;
                }
                _ => {
                    //for rock
                    score += 1;
                    //for win
                    score += 6;
                }
            },
        }
    }
    println!("{score}");
}
