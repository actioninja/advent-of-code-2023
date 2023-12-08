use rayon::prelude::*;
use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;
advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coordinate(pub [char; 3]);

impl Coordinate {
    pub fn last(&self) -> char {
        self.0[2]
    }
    pub fn starting_node(&self) -> Option<[char; 2]> {
        if self.0[2] == 'A' {
            Some([self.0[0], self.0[1]])
        } else {
            None
        }
    }
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = ['0'; 3];
        for (i, c) in s.chars().enumerate() {
            chars[i] = c;
        }
        Ok(Coordinate(chars))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction {
    pub left: Coordinate,
    pub right: Coordinate,
}

impl Instruction {
    pub fn pick(&self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .trim()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(',');
        let left = parts.next().unwrap().trim().parse().unwrap();
        let right = parts.next().unwrap().trim().parse().unwrap();
        Ok(Instruction { left, right })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub sequence: Vec<Direction>,
    pub coordinates: BTreeMap<Coordinate, Instruction>,
}

impl Map {
    pub fn steps_to_zzz(&self) -> usize {
        let mut steps = 0;
        let mut current = "AAA".parse::<Coordinate>().unwrap();
        for direction in self.sequence.iter().cycle() {
            steps += 1;
            let instruction = self.coordinates.get(&current).unwrap();
            current = instruction.pick(*direction);
            if current == "ZZZ".parse().unwrap() {
                break;
            }
        }
        steps
    }

    pub fn starting_nodes(&self) -> HashSet<Coordinate> {
        let mut nodes = HashSet::new();
        for (coordinate, _) in self.coordinates.iter() {
            if coordinate.starting_node().is_some() {
                nodes.insert(*coordinate);
            }
        }
        nodes
    }

    pub fn length_to_z(&self, starting_node: Coordinate) -> usize {
        let mut steps = 0;
        let mut current = starting_node;
        for direction in self.sequence.iter().cycle() {
            steps += 1;
            let instruction = self.coordinates.get(&current).unwrap();
            current = instruction.pick(*direction);
            if current.last() == 'Z' {
                break;
            }
        }
        steps
    }

    pub fn steps_to_all_z(&self) -> usize {
        let starting_nodes = self.starting_nodes();
        let steps_array = starting_nodes
            .par_iter()
            .map(|&starting_node| self.length_to_z(starting_node))
            .collect::<Vec<_>>();
        println!("steps_array: {:?}", steps_array);
        lcm_slice(&steps_array)
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let sequence = lines
            .next()
            .unwrap()
            .chars()
            .map(Direction::from)
            .collect::<Vec<_>>();

        let coordinates = lines
            .filter(|x| !x.is_empty())
            .map(|x| {
                let mut parts = x.split('=');
                let coordinate: Coordinate = parts.next().unwrap().trim().parse().unwrap();
                let instruction: Instruction = parts.next().unwrap().trim().parse().unwrap();
                (coordinate, instruction)
            })
            .collect();

        Ok(Map {
            sequence,
            coordinates,
        })
    }
}

/// calculate the greatest common divisor of two numbers using the euclidean algorithm
pub fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        if b < a {
            std::mem::swap(&mut b, &mut a);
        }
        b %= a;
    }
    a
}

pub fn least_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / greatest_common_divisor(a, b)
}

// this doesn't work and I'm not sure why, I just ended up using a calculator on the intermediate result
pub fn lcm_slice(slice: &[usize]) -> usize {
    if slice.len() == 1 {
        slice[0]
    } else if slice.len() == 2 {
        least_common_multiple(slice[0], slice[1])
    } else {
        least_common_multiple(slice[0], lcm_slice(&slice[1..]))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input.parse::<Map>().unwrap();
    let steps = map.steps_to_zzz();
    Some(steps as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input.parse::<Map>().unwrap();
    let steps = map.steps_to_all_z();
    Some(steps as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("result: {:?}", lcm_slice(&[27, 43, 81]));
        assert_eq!(result, Some(6));
    }
}
