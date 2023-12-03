mod day;
pub mod template;

pub use day::*;
use std::collections::HashSet;

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
