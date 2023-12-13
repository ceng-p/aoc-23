use std::i32;

pub fn run_part1(input_lines: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in input_lines {
        let mut parsed_line = String::new();
        for c in line.chars() {
            if let Some(_) = c.to_digit(10) {
                parsed_line += c.to_string().as_str();
                break;
            }
        }
        for c in line.chars().rev() {
            if let Some(_) = c.to_digit(10) {
                parsed_line += c.to_string().as_str();
                break;
            }
        }
        println!("line: {}", parsed_line);
        sum += parsed_line.parse::<i32>().unwrap();
        println!("sum: {}", sum);
    }
    return sum;
}

pub fn run_part2(input_lines: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in input_lines {}

    return sum;
}
