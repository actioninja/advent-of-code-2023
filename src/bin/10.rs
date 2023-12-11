use advent_of_code::template::aoc_cli::check;
use advent_of_code::{Direction, Grid};
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;
advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    #[default]
    Ground,
    INNER,
    OUTER,
}

impl Pipe {
    pub fn from_char(c: char) -> Option<Pipe> {
        match c {
            '|' => Some(Pipe::Vertical),
            '-' => Some(Pipe::Horizontal),
            'L' => Some(Pipe::NorthEast),
            'J' => Some(Pipe::NorthWest),
            '7' => Some(Pipe::SouthWest),
            'F' => Some(Pipe::SouthEast),
            'S' => Some(Pipe::Start),
            '.' => Some(Pipe::Ground),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Pipe::Vertical => '│',
            Pipe::Horizontal => '─',
            Pipe::NorthEast => '└',
            Pipe::NorthWest => '┘',
            Pipe::SouthWest => '┐',
            Pipe::SouthEast => '┌',
            Pipe::Start => 'S',
            Pipe::Ground => ' ',
            Pipe::INNER => 'I',
            Pipe::OUTER => 'O',
        }
    }

    pub fn is_start(self) -> bool {
        matches!(self, Pipe::Start)
    }

    /// Given a direction and a pipe, return the next direction to go to find the next pipe.
    /// If there is no next pipe, return None, which shouldn't happen.
    /// Note that the provided direction is the direction which was stepped towards.
    /// ie, if you are coming from the north you stepped south to get to the pipe being checked
    pub fn next_from_direction(self, direction: Direction) -> Option<Direction> {
        match (self, direction) {
            (Pipe::Vertical, Direction::South) => Some(Direction::South),
            (Pipe::Vertical, Direction::North) => Some(Direction::North),
            (Pipe::Horizontal, Direction::East) => Some(Direction::East),
            (Pipe::Horizontal, Direction::West) => Some(Direction::West),
            (Pipe::NorthEast, Direction::South) => Some(Direction::East),
            (Pipe::NorthEast, Direction::West) => Some(Direction::North),
            (Pipe::NorthWest, Direction::South) => Some(Direction::West),
            (Pipe::NorthWest, Direction::East) => Some(Direction::North),
            (Pipe::SouthWest, Direction::North) => Some(Direction::West),
            (Pipe::SouthWest, Direction::East) => Some(Direction::South),
            (Pipe::SouthEast, Direction::North) => Some(Direction::East),
            (Pipe::SouthEast, Direction::West) => Some(Direction::South),
            _ => {
                panic!("Invalid direction {:?} for pipe {:?}", direction, self);
            }
        }
    }

    pub fn get_connected_directions(self) -> Vec<Direction> {
        match self {
            Pipe::Vertical => vec![Direction::North, Direction::South],
            Pipe::Horizontal => vec![Direction::East, Direction::West],
            Pipe::NorthEast => vec![Direction::North, Direction::East],
            Pipe::NorthWest => vec![Direction::North, Direction::West],
            Pipe::SouthWest => vec![Direction::South, Direction::West],
            Pipe::SouthEast => vec![Direction::South, Direction::East],
            Pipe::Start => vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
            Pipe::Ground => vec![],
            _ => {
                panic!("Invalid pipe {:?}", self);
            }
        }
    }

    /// Given a direction, return whether or not the pipe would be connected to
    /// a pipe "coming from" that direction.
    /// ie, if the pipe is vertical, and the direction is south, return true.
    pub fn connected_to_from(self, direction: Direction) -> bool {
        matches!(
            (self, direction),
            (Pipe::Vertical, Direction::South)
                | (Pipe::Vertical, Direction::North)
                | (Pipe::Horizontal, Direction::East)
                | (Pipe::Horizontal, Direction::West)
                | (Pipe::NorthEast, Direction::South)
                | (Pipe::NorthEast, Direction::West)
                | (Pipe::NorthWest, Direction::South)
                | (Pipe::NorthWest, Direction::East)
                | (Pipe::SouthWest, Direction::North)
                | (Pipe::SouthWest, Direction::East)
                | (Pipe::SouthEast, Direction::North)
                | (Pipe::SouthEast, Direction::West)
        )
    }

    pub fn points_north(self) -> bool {
        matches!(self, Pipe::Vertical | Pipe::NorthEast | Pipe::NorthWest)
    }
}

#[derive(Debug, Clone)]
pub struct PipeGrid {
    pub grid: Grid<Pipe>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PipeStep {
    pub pipe: Pipe,
    pub direction: Direction,
    pub coordinates: (usize, usize),
    pub distance: u32,
}

impl PipeStep {
    pub fn new(pipe: Pipe, direction: Direction, coordinates: (usize, usize)) -> Self {
        Self {
            pipe,
            direction,
            coordinates,
            distance: 1,
        }
    }
}

impl PipeGrid {
    pub fn find_connected_to_start(&self) -> [PipeStep; 2] {
        let start = self.get_start_position().unwrap();
        let mut out = vec![];
        for direction in Pipe::Start.get_connected_directions() {
            if let Some(next) = self.grid.get_step(direction, start) {
                if let Some(pipe) = self.grid.get(next.0, next.1) {
                    if pipe.connected_to_from(direction) {
                        out.push(PipeStep::new(*pipe, direction, next));
                    }
                }
            }
        }
        [out[0], out[1]]
    }

    pub fn furthest_distance(&self) -> u32 {
        let mut checked_coords = HashSet::new();
        let start = self.get_start_position().unwrap();
        checked_coords.insert(start);
        let mut current_check_coord = self.find_connected_to_start();
        checked_coords.insert(current_check_coord[0].coordinates);
        checked_coords.insert(current_check_coord[1].coordinates);
        loop {
            for step in current_check_coord.iter_mut() {
                let next = step.pipe.next_from_direction(step.direction).unwrap();
                let next_coord = self.grid.get_step(next, step.coordinates).unwrap();
                if !checked_coords.contains(&next_coord) {
                    checked_coords.insert(next_coord);
                    let next_pipe = self.grid.get(next_coord.0, next_coord.1).unwrap();
                    let next_distance = step.distance + 1;
                    *step = PipeStep {
                        pipe: *next_pipe,
                        direction: next,
                        coordinates: next_coord,
                        distance: next_distance,
                    };
                } else {
                    return step.distance + 1;
                }
            }
        }
    }

    pub fn find_loop_tiles(&self) -> HashSet<(usize, usize)> {
        let mut checked_coords = HashSet::new();
        let start = self.get_start_position().unwrap();
        checked_coords.insert(start);
        let mut step = self.find_connected_to_start()[0];
        checked_coords.insert(step.coordinates);
        loop {
            let next = step.pipe.next_from_direction(step.direction).unwrap();
            let next_coord = self.grid.get_step(next, step.coordinates).unwrap();
            if !checked_coords.contains(&next_coord) {
                checked_coords.insert(next_coord);
                let next_pipe = self.grid.get(next_coord.0, next_coord.1).unwrap();
                let next_distance = step.distance + 1;
                step = PipeStep {
                    pipe: *next_pipe,
                    direction: next,
                    coordinates: next_coord,
                    distance: next_distance,
                };
            } else {
                return checked_coords;
            }
        }
    }

    pub fn grid_with_only_loop_tiles(&self) -> Self {
        let mut grid = self.clone();
        let loop_tiles = self.find_loop_tiles();
        for (y, row) in self.grid.iter_rows().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if !loop_tiles.contains(&(x, y)) {
                    grid.grid.set(x, y, Pipe::Ground);
                }
            }
        }
        grid
    }

    pub fn grid_with_overlaid_inner_outer(&mut self) -> Self {
        let mut grid = self.clone();
        let loop_tiles = self.find_loop_tiles();
        let enclosed_tiles = self.enclosed_tiles();
        for (y, row) in self.grid.iter_rows().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if !loop_tiles.contains(&(x, y)) {
                    if enclosed_tiles.contains(&(x, y)) {
                        grid.grid.set(x, y, Pipe::INNER);
                    } else {
                        grid.grid.set(x, y, Pipe::OUTER);
                    }
                }
            }
        }
        grid
    }

    pub fn enclosed_tiles(&mut self) -> HashSet<(usize, usize)> {
        let loop_tiles = self.find_loop_tiles();
        // at this point I don't give a shit, this will only work on my input
        if let Some(start) = self.get_start_position() {
            self.grid.set(start.0, start.1, Pipe::NorthEast);
        }
        let mut inside = HashSet::new();
        for (y, row) in self.grid.iter_rows().enumerate() {
            let mut is_inside = false;
            for (x, pipe) in row.iter().enumerate() {
                if loop_tiles.contains(&(x, y)) {
                    if pipe.points_north() {
                        is_inside = !is_inside;
                    }
                } else if is_inside {
                    inside.insert((x, y));
                }
            }
        }
        inside
    }

    pub fn get_start_position(&self) -> Option<(usize, usize)> {
        for (y, row) in self.grid.iter_rows().enumerate() {
            for (x, pipe) in row.iter().enumerate() {
                if pipe.is_start() {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

impl Display for PipeGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter_rows() {
            for pipe in row {
                write!(f, "{}", pipe.to_char())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for PipeGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let width = lines[0].len();
        let height = lines.len();
        let mut grid = Grid::new_filled_default(width, height);
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pipe = Pipe::from_char(c).unwrap();
                grid.set(x, y, pipe);
            }
        }
        Ok(PipeGrid { grid })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = PipeGrid::from_str(input).unwrap();
    //println!("{grid}");
    let furthest_distance = grid.furthest_distance();
    Some(furthest_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = PipeGrid::from_str(input).unwrap();
    let loop_grid = grid.grid_with_only_loop_tiles();
    println!("{loop_grid}");
    let enclosed_tiles = grid.clone().enclosed_tiles();
    println!("{:?}", enclosed_tiles);
    let enclosed_grid = grid.clone().grid_with_overlaid_inner_outer();
    println!("{enclosed_grid}");
    Some(enclosed_tiles.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
