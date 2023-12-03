use advent_of_code::surrounding_indexes;
use grid::Grid;
use std::collections::HashMap;
advent_of_code::solution!(3);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GridSpot {
    Num(char),
    Symbol,
    Gear,
    #[default]
    Empty,
}

impl GridSpot {
    pub fn from_char(input: char) -> Option<Self> {
        match input {
            '=' | '#' | '%' | '/' | '+' | '-' | '&' | '$' | '@' => Some(Self::Symbol),
            '*' => Some(Self::Gear),
            '.' => Some(Self::Empty),
            _ => Some(Self::Num(input)),
        }
    }

    pub fn unwrap_num(&self) -> char {
        match self {
            Self::Num(c) => *c,
            _ => panic!("unwrap_num called on non-num"),
        }
    }

    pub fn is_symbol(self) -> bool {
        matches!(self, Self::Symbol | Self::Gear)
    }
}

fn parse_to_grid(input: &str) -> Grid<GridSpot> {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines.first().unwrap().len();
    let height = lines.len();
    let mut grid = Grid::new(width, height);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let grid_spot = GridSpot::from_char(c).unwrap();
            let grid_ref = grid.get_mut(y, x).unwrap();
            *grid_ref = grid_spot;
        }
    }
    grid
}

fn resolve_number(grid: &Grid<GridSpot>, pos: (usize, usize)) -> u32 {
    let mut vec: Vec<GridSpot> = Vec::new();

    let mut pos = pos;

    while let Some(grid_spot) = grid.get(pos.1, pos.0) {
        if let GridSpot::Num(_) = grid_spot {
            vec.push(*grid_spot);
            pos.0 += 1;
        } else {
            break;
        };
    }

    vec.iter()
        .map(|x| x.unwrap_num())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn valid_search_for_number(grid: &Grid<GridSpot>, pos: (usize, usize)) -> bool {
    let mut current_pos = pos;
    while let Some(grid_spot) = grid.get(current_pos.1, current_pos.0) {
        if let GridSpot::Num(_) = grid_spot {
            let surrounding = surrounding_indexes(current_pos, grid.size());
            let has_symbol = surrounding.iter().any(|x| {
                let grid_spot = grid.get(x.1, x.0).unwrap();
                grid_spot.is_symbol()
            });
            if has_symbol {
                return true;
            }
            current_pos.0 += 1;
        } else {
            break;
        };
    }
    false
}

fn find_numbers_in_line(grid: &Grid<GridSpot>, line_num: usize) -> Vec<(usize, usize)> {
    let mut vector = Vec::new();
    let mut line = grid.iter_row(line_num).enumerate();
    loop {
        if let Some((x, grid_spot)) = line.next() {
            if let GridSpot::Num(_) = grid_spot {
                vector.push((x, line_num));
                while let Some((_, grid_spot)) = line.next() {
                    if !matches!(grid_spot, GridSpot::Num(_)) {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
    vector
}

pub fn find_number_starts(grid: &Grid<GridSpot>) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    for y in 0..grid.size().1 {
        vec.append(&mut find_numbers_in_line(grid, y));
    }
    vec
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let numbers = find_number_starts(&grid);
    let valid_numbers = numbers
        .iter()
        .filter(|x| valid_search_for_number(&grid, **x))
        .collect::<Vec<_>>();
    let resolved_numbers = valid_numbers
        .iter()
        .map(|x| resolve_number(&grid, **x))
        .collect::<Vec<_>>();
    Some(resolved_numbers.iter().sum())
}

pub fn potential_gear(grid: &Grid<GridSpot>, pos: (usize, usize)) -> Option<(usize, usize)> {
    let mut current_pos = pos;
    while let Some(grid_spot) = grid.get(current_pos.1, current_pos.0) {
        if let GridSpot::Num(_) = grid_spot {
            let surrounding = surrounding_indexes(current_pos, grid.size());
            let mut gear_pos = (0, 0);
            let has_gear = surrounding.iter().any(|x| {
                let grid_spot = grid.get(x.1, x.0).unwrap();
                gear_pos = *x;
                matches!(grid_spot, GridSpot::Gear)
            });
            if has_gear {
                return Some(gear_pos);
            }
            current_pos.0 += 1;
        } else {
            break;
        };
    }
    None
}
pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let numbers = find_number_starts(&grid);
    let valid_numbers = numbers
        .iter()
        .filter_map(|x| potential_gear(&grid, *x).map(|inner| (*x, inner)))
        .collect::<Vec<_>>();
    let mut gears = HashMap::new();
    let mut ratios = Vec::new();
    for (num_pos, gear_pos) in valid_numbers {
        let num = resolve_number(&grid, num_pos);
        if let Some(gear) = gears.get(&gear_pos) {
            ratios.push(num * gear);
        } else {
            gears.insert(gear_pos, num);
        }
    }
    Some(ratios.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
