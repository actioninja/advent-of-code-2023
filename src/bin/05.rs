use std::collections::{BTreeMap, HashMap};
use std::iter::Map;
use std::str::FromStr;
advent_of_code::solution!(5);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapping {
    pub dest: (u32, u32),
    pub source: (u32, u32),
    pub range: u32,
}

impl Mapping {
    pub fn map(&self, number: u32) -> Option<u32> {
        if number >= self.source.0 && number <= self.source.1 {
            let offset = number - self.source.0;
            Some(self.dest.0 + offset)
        } else {
            None
        }
    }
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split_whitespace().collect();
        if split.len() != 3 {
            return Err(());
        }
        let dest = split[0].parse().unwrap();
        let source = split[1].parse().unwrap();
        let range = split[2].parse().unwrap();

        Ok(Mapping {
            dest: (dest, dest + (range - 1)),
            source: (source, source + (range - 1)),
            range,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mappings {
    pub from: String,
    pub to: String,
    pub mappings: Vec<Mapping>,
}

impl Mappings {
    pub fn map(&self, number: u32) -> u32 {
        let mut number = number;
        for mapping in &self.mappings {
            if let Some(mapped) = mapping.map(number) {
                number = mapped;
                break;
            }
        }
        number
    }
}

impl FromStr for Mappings {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines
            .next()
            .unwrap()
            .strip_suffix(" map:")
            .unwrap()
            .split("-to-");
        let from = first_line.clone().next().unwrap().to_string();
        let to = first_line.clone().last().unwrap().to_string();

        let mut mappings = vec![];
        for line in lines {
            let mapping = line.parse::<Mapping>().unwrap();
            mappings.push(mapping);
        }
        Ok(Mappings { from, to, mappings })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterMap {
    pub maps: HashMap<String, Mappings>,
}

impl MasterMap {
    pub fn new(in_vec: &[Mappings]) -> Self {
        let mut maps = HashMap::new();
        for mapping in in_vec {
            maps.insert(mapping.to.clone(), mapping.clone());
        }
        MasterMap { maps }
    }

    pub fn map(&self, to: &str, number: u32) -> u32 {
        let mappings = self.maps.get(to).unwrap();
        mappings.map(number)
    }

    pub fn map_all(&self, number: u32) -> Results {
        let seed = number;
        let soil = self.map("soil", seed);
        let fertilizer = self.map("fertilizer", soil);
        let water = self.map("water", fertilizer);
        let light = self.map("light", water);
        let temperature = self.map("temperature", light);
        let humidity = self.map("humidity", temperature);
        let location = self.map("location", humidity);
        Results {
            seed,
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
            location,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Results {
    pub seed: u32,
    pub soil: u32,
    pub fertilizer: u32,
    pub water: u32,
    pub light: u32,
    pub temperature: u32,
    pub humidity: u32,
    pub location: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut groups = input.split("\n\n");
    let seeds = groups
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mappings: Vec<_> = groups.map(|g| g.parse::<Mappings>().unwrap()).collect();
    let master_map = MasterMap::new(&mappings);
    println!("{master_map:#?}");

    let results = seeds
        .iter()
        .map(|seed| master_map.map_all(*seed))
        .collect::<Vec<_>>();

    let min = results.iter().map(|r| r.location).min().unwrap();
    Some(min)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut groups = input.split("\n\n");

    /*
    let seeds = groups
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
     */

    let seeds: Vec<_> = groups
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .collect();
    println!("Seed Ranges (pre parse): {:#?}", seeds);

    let seeds_ranges: Vec<_> = seeds
        .chunks(2)
        .map(|window| {
            let low = window[0].parse::<u32>().unwrap();
            let high = low + (window[1].parse::<u32>().unwrap() - 1);
            (low, high)
        })
        .collect();
    println!("Seed Ranges: {:#?}", seeds_ranges);

    let mappings: Vec<_> = groups.map(|g| g.parse::<Mappings>().unwrap()).collect();
    let master_map = MasterMap::new(&mappings);

    let mut lowest_so_far = u32::MAX;
    for (low, high) in seeds_ranges {
        println!("Checkpoint! Starting {} - {}", low, high);
        for i in low..=high {
            let results = master_map.map_all(i);
            if results.location < lowest_so_far {
                println!("Found new lowest: {}", results.location);
                lowest_so_far = results.location;
            }
        }
    }
    Some(lowest_so_far)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn verify_order() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let mut groups = input.split("\n\n");
        let seeds = groups
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mappings: Vec<_> = groups.map(|g| g.parse::<Mappings>().unwrap()).collect();
        let master_map = MasterMap::new(&mappings);
        println!("{master_map:#?}");

        let result = master_map.map_all(14);

        let expected = Results {
            seed: 14,
            soil: 14,
            fertilizer: 53,
            water: 49,
            light: 42,
            temperature: 42,
            humidity: 43,
            location: 43,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
