use std::fmt::format;
advent_of_code::solution!(1);

const NUMS_TEXT: [(&'static str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn brute_force_at_position(input: &str, position: usize) -> Option<char> {
    for (text, num) in NUMS_TEXT.iter() {
        if input[position..].starts_with(text) {
            return Some(*num);
        }
    }
    None
}

fn turn_text_to_chars(input: &str) -> Vec<char> {
    let mut chars = Vec::new();
    let mut position = 0;
    while position < input.len() {
        if let Some(num) = input.chars().nth(position) {
            if num.is_numeric() {
                chars.push(num);
                position += 1;
                continue;
            }
        }
        let num = brute_force_at_position(input, position);
        if let Some(num) = num {
            chars.push(num);
        }
        position += 1;
    }
    chars
}

fn parse_line(input: &str) -> (char, char) {
    let first = input.chars().find(|c| c.is_numeric()).unwrap();
    let last = input.chars().filter(|c| c.is_numeric()).last().unwrap();
    (first, last)
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(parse_line)
        .map(|(first, second)| format!("{first}{second}"))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = parse_input(input).iter().map(|s| s.parse::<u32>().unwrap()).sum();
    Some(sum)
}

fn parse_line_2(input: &str) -> (char, char) {
    let nums = turn_text_to_chars(input);

    println!("given input: {input}; parsed {nums:?}");
    (*nums.first().unwrap(), *nums.last().unwrap())
}

fn parse_input_2(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(parse_line_2)
        .map(|(first, second)| {
            println!("{first}{second}");
            format!("{first}{second}")
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = parse_input_2(input).iter().map(|s| s.parse::<u32>().unwrap()).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
