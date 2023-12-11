mod day;
pub mod template;

pub use day::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new_filled_default(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Default::default(); width * height],
        }
    }

    pub fn new_with_same_size<Q: Clone + Default>(&self) -> Grid<Q> {
        Grid {
            width: self.width,
            height: self.height,
            data: vec![Default::default(); self.width * self.height],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.data.resize(width * height, Default::default());
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: Vec::with_capacity(width * height),
        }
    }

    pub fn new_filled(width: usize, height: usize, value: T) -> Self {
        Self {
            width,
            height,
            data: vec![value; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.data.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.data.get_mut(y * self.width + x)
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.width)
    }

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.chunks_mut(self.width)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.data[y * self.width + x] = value;
    }

    pub fn set_all(&mut self, value: T) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set(x, y, value.clone());
            }
        }
    }

    pub fn surrounding_indexes(&self, pos: (usize, usize)) -> HashSet<(usize, usize)> {
        let (x, y) = pos;
        let mut set = HashSet::new();
        if x > 0 {
            set.insert((x - 1, y));
        }
        if x > 0 && y > 0 {
            set.insert((x - 1, y - 1));
        }
        if y > 0 {
            set.insert((x, y - 1));
        }
        if y > 0 && x < self.width - 1 {
            set.insert((x + 1, y - 1));
        }
        if x < self.width - 1 {
            set.insert((x + 1, y));
        }
        if x < self.width - 1 && y < self.height - 1 {
            set.insert((x + 1, y + 1));
        }
        if y < self.height - 1 {
            set.insert((x, y + 1));
        }
        if y < self.height - 1 && x > 0 {
            set.insert((x - 1, y + 1));
        }
        set
    }

    pub fn surrounding_cardinal_indexes(&self, pos: (usize, usize)) -> HashSet<(usize, usize)> {
        let (x, y) = pos;
        let mut set = HashSet::new();
        if x > 0 {
            set.insert((x - 1, y));
        }
        if y > 0 {
            set.insert((x, y - 1));
        }
        if x < self.width - 1 {
            set.insert((x + 1, y));
        }
        if y < self.height - 1 {
            set.insert((x, y + 1));
        }
        set
    }

    pub fn is_adjacent_to_edge(&self, pos: (usize, usize)) -> bool {
        let (x, y) = pos;
        x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1
    }

    pub fn get_step(&self, direction: Direction, coords: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = coords;
        match direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::East => {
                if x == self.width - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::South => {
                if y == self.height - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Direction::NorthEast => {
                if x == self.width - 1 || y == 0 {
                    None
                } else {
                    Some((x + 1, y - 1))
                }
            }
            Direction::NorthWest => {
                if x == 0 || y == 0 {
                    None
                } else {
                    Some((x - 1, y - 1))
                }
            }
            Direction::SouthEast => {
                if x == self.width - 1 || y == self.height - 1 {
                    None
                } else {
                    Some((x + 1, y + 1))
                }
            }
            Direction::SouthWest => {
                if x == 0 || y == self.height - 1 {
                    None
                } else {
                    Some((x - 1, y + 1))
                }
            }
        }
    }
}

pub fn surrounding_indexes(
    pos: (usize, usize),
    dims_of_grid: (usize, usize),
) -> HashSet<(usize, usize)> {
    let (x, y) = pos;
    let (height, width) = dims_of_grid;
    let mut set = HashSet::new();
    if x > 0 {
        set.insert((x - 1, y));
    }
    if x > 0 && y > 0 {
        set.insert((x - 1, y - 1));
    }
    if y > 0 {
        set.insert((x, y - 1));
    }
    if y > 0 && x < width - 1 {
        set.insert((x + 1, y - 1));
    }
    if x < width - 1 {
        set.insert((x + 1, y));
    }
    if x < width - 1 && y < height - 1 {
        set.insert((x + 1, y + 1));
    }
    if y < height - 1 {
        set.insert((x, y + 1));
    }
    if y < height - 1 && x > 0 {
        set.insert((x - 1, y + 1));
    }
    set
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_surrounding_indexes() {
        let set = surrounding_indexes((0, 0), (3, 3));
        assert_eq!(set.len(), 3);
        assert!(set.contains(&(0, 1)));
        assert!(set.contains(&(1, 0)));
        assert!(set.contains(&(1, 1)));

        let set = surrounding_indexes((1, 1), (3, 3));
        assert_eq!(set.len(), 8);
        assert!(set.contains(&(0, 0)));
        assert!(set.contains(&(0, 1)));
        assert!(set.contains(&(0, 2)));
        assert!(set.contains(&(1, 0)));
        assert!(set.contains(&(1, 2)));
        assert!(set.contains(&(2, 0)));
        assert!(set.contains(&(2, 1)));
        assert!(set.contains(&(2, 2)));

        let set = surrounding_indexes((2, 2), (3, 3));
        assert_eq!(set.len(), 3);
        assert!(set.contains(&(1, 1)));
        assert!(set.contains(&(1, 2)));
        assert!(set.contains(&(2, 1)));
    }
}
