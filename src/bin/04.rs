use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

fn parse_line(line: &str) -> (HashSet<u32>, Vec<u32>) {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        return (HashSet::new(), Vec::new());
    }

    let left_numbers = parts[0]
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect::<HashSet<_>>();

    let right_numbers = parts[1]
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect::<Vec<_>>();

    (left_numbers, right_numbers)
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| parse_line(line))
        .map(|(wins, mine)| {
            let overlaps = mine
                .iter()
                .filter(|num| wins.contains(num))
                .collect::<Vec<_>>()
                .len();

            if overlaps > 0 {
                2u32.pow(overlaps as u32 - 1)
            } else {
                0
            }
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut dup_counts: HashMap<usize, u32> = HashMap::new();

    input
        .lines()
        .map(|line| parse_line(line))
        .enumerate()
        .map(|(idx, (wins, mine))| {
            let overlaps = mine
                .iter()
                .filter(|num| wins.contains(num))
                .collect::<Vec<_>>()
                .len();
            let curr_count: u32 = dup_counts.get(&idx).unwrap_or(&0) + 1;

            for i in 1..=overlaps {
                *dup_counts.entry(&idx + i).or_insert(0) += curr_count;
            }
            curr_count
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
