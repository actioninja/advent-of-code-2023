advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "red" => Some(Self::Red),
            "green" => Some(Self::Green),
            "blue" => Some(Self::Blue),
            _ => None,
        }
    }
}

impl std::str::FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pull {
    pub number: u32,
    pub color: Color,
}

impl Pull {
    pub fn from_str(input: &str) -> Option<Self> {
        let mut iter = input.trim().split_whitespace();
        let number = iter.next()?.parse().ok()?;
        let color = iter.next()?.parse().ok()?;
        Some(Self { number, color })
    }
}

impl std::str::FromStr for Pull {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set {
    pub red: Option<Pull>,
    pub green: Option<Pull>,
    pub blue: Option<Pull>,
}

impl Set {
    pub fn from_str(input: &str) -> Option<Self> {
        let mut iter = input.trim().split(',');
        let mut red = None;
        let mut green = None;
        let mut blue = None;
        while let Some(x) = iter.next() {
            match x.parse::<Pull>() {
                Ok(pull) => match pull.color {
                    Color::Red => red = Some(pull),
                    Color::Green => green = Some(pull),
                    Color::Blue => blue = Some(pull),
                },
                Err(_) => return None,
            }
        }
        Some(Self { red, green, blue })
    }
}

impl std::str::FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub num: u32,
    pub sets: Vec<Set>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colors {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Colors {
    fn powers(self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Game {
    pub fn from_str(input: &str) -> Option<Self> {
        let mut game_split = input.trim().split(':');
        let game_info = game_split.next()?;
        let game_number = game_info.trim().split_whitespace().last()?.parse();
        let game_results = game_split.next()?;
        let mut sets = Vec::new();
        for set in game_results.trim().split(';') {
            sets.push(set.parse().ok()?);
        }
        Some(
            Self {
                num: game_number.ok()?,
                sets,
            }
        )
    }

    pub fn total_by_color(&self) -> Colors {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for set in &self.sets {
            if let Some(pull) = &set.red {
                red += pull.number;
            }
            if let Some(pull) = &set.green {
                green += pull.number;
            }
            if let Some(pull) = &set.blue {
                blue += pull.number;
            }
        }
        Colors { red, green, blue }
    }
}

fn valid_set_1(set: &Set) -> bool {
    if let Some(pull) = &set.red {
        if pull.number > 12 {
            return false;
        }
    }
    if let Some(pull) = &set.green {
        if pull.number > 13 {
            return false;
        }
    }
    if let Some(pull) = &set.blue {
        if pull.number > 14 {
            return false;
        }
    }
    true
}

fn filter_possible_1(game: &Game) -> bool {
    for set in &game.sets {
        if !valid_set_1(set) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| Game::from_str(line).unwrap()).filter(|game| filter_possible_1(game)).map(|game| game.num).sum())
}

fn min_colors(game: &Game) -> Colors {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for set in &game.sets {
        if let Some(pull) = &set.red {
            if pull.number > red {
                red = pull.number;
            }
        }
        if let Some(pull) = &set.green {
            if pull.number > green {
                green = pull.number;
            }
        }
        if let Some(pull) = &set.blue {
            if pull.number > blue {
                blue = pull.number;
            }
        }
    }
    Colors { red, green, blue }
}



pub fn part_two(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .map(|game| min_colors(&game).powers())
        .sum::<u32>())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        let game = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        println!("{:?}", game);
        println!("{:?}", game.total_by_color());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
