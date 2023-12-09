advent_of_code::solution!(9);

pub fn all_zeroes(seq: &[i32]) -> bool {
    seq.iter().all(|&x| x == 0)
}

pub fn build_sequence_tree(seq: &[i32]) -> Vec<Vec<i32>> {
    let mut tree = vec![seq.to_vec()];
    let mut working_seq: Vec<i32> = tree.first().unwrap().clone();
    let mut next_seq: Vec<i32> = vec![];
    while !all_zeroes(&working_seq) {
        for window in working_seq.windows(2) {
            let a = window[0];
            let b = window[1];
            next_seq.push(b - a);
        }
        tree.push(next_seq.to_vec());
        working_seq = next_seq;
        next_seq = vec![];
    }
    tree
}

pub fn predict_next(seq: &[i32]) -> i32 {
    let tree = build_sequence_tree(seq);
    //print_sequence_tree(&tree);
    let mut prev_last_value = 0;
    for tree_seq in tree.iter().rev() {
        let last_value = tree_seq.last().unwrap();
        prev_last_value += last_value;
    }
    prev_last_value
}

fn print_sequence_tree(sequences: &[Vec<i32>]) {
    for (i, seq) in sequences.iter().enumerate() {
        for _ in 0..i {
            print!("  ");
        }
        for (j, x) in seq.iter().enumerate() {
            if j == seq.len() - 1 {
                println!("{}", x);
            } else {
                print!("{}  ", x);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let sequences = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let sum = sequences.iter().map(|x| predict_next(x)).sum::<i32>();
    Some(sum)
}

pub fn predict_previous(seq: &[i32]) -> i32 {
    let tree = build_sequence_tree(seq);
    //print_sequence_tree(&tree);
    let mut prev_first_value = 0;
    for tree_seq in tree.iter().rev() {
        let first_value = tree_seq.first().unwrap();
        prev_first_value = first_value - prev_first_value;
    }
    prev_first_value
}

pub fn part_two(input: &str) -> Option<i32> {
    let sequences = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let sum = sequences.iter().map(|x| predict_previous(x)).sum::<i32>();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
