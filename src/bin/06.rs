use std::str::FromStr;
advent_of_code::solution!(6);
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Race {
    pub length: u32,
    pub record: u32,
}

pub fn parse_races(in_str: &str) -> Vec<Race> {
    let mut lines = in_str.lines();
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let records: Vec<u32> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    times
        .iter()
        .zip(records.iter())
        .map(|(time, record)| Race {
            length: *time,
            record: *record,
        })
        .collect()
}

fn brute_force_race(race: Race) -> usize {
    let record = race.record;
    let length = race.length;
    (1..length)
        .into_par_iter()
        .map(|x| {
            let running_time = length - x;
            running_time * x
        })
        .filter(|&x| x > record)
        .count()
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse_races(input);
    let mut records = Vec::new();
    for race in races {
        let winning_records = brute_force_race(race);
        records.push(winning_records as u32);
    }
    Some(records.iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .replace(' ', "")
        .strip_prefix("Time:")
        .unwrap()
        .parse()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .replace(' ', "")
        .strip_prefix("Distance:")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let winning_records = brute_force_race(Race {
        length: time as u32,
        record: distance as u32,
    });
    Some(winning_records as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
