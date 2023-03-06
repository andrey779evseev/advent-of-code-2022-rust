use std::vec;

fn main() {
    let lines = include_str!("input.txt").lines();

    let mut current_path = "/".to_string();

    let mut folders: Vec<(String, i32)> = vec![];

    for line in lines {
        let splitted: Vec<&str> = line.split_whitespace().collect();
        match splitted[0] {
            "$" => match splitted[1] {
                "cd" => match splitted[2] {
                    "/" => current_path = "/".to_string(),
                    ".." => {
                        let mut path: Vec<_> = current_path.split('/').collect();
                        path.pop();
                        current_path = path.join("/");
                    }
                    folder => {
                        current_path = if current_path == "/" {
                            "/".to_owned() + folder
                        } else {
                            current_path + "/" + folder
                        }
                    }
                },
                _ => {}
            },
            "dir" => {}
            size => {
                let size = size.parse::<i32>().unwrap();
                let splitted_path: Vec<_> = current_path.split("/").collect();

                for i in 0..splitted_path.len() {
                    let path = &splitted_path[0..=i].join("");
                    let item = folders.iter_mut().find(|f| f.0 == *path);
                    match item {
                        Some(item) => item.1 += size,
                        None => folders.push((path.clone(), size)),
                    }
                }
            }
        }
    }

    let needed_space = 30_000_000 - (70_000_000 - folders.iter().find(|f| f.0 == "").unwrap().1);

    println!(
        "{}",
        folders
            .iter()
            .map(|(_, size)| *size)
            .filter(|size| *size >= needed_space)
            .min()
            .unwrap()
    )
}
