use std::io::{self};

pub fn read_input() -> Vec<String> {
    let input = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    input
}
