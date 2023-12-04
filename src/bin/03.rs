use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

struct Number {
    start: usize,
    end: usize,
    value: u32,
}

fn create_number(num_str: &str, start: usize, end: usize) -> Option<Number> {
    num_str
        .parse::<u32>()
        .ok()
        .map(|value| Number { start, end, value })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut number_coords: HashMap<usize, Vec<Number>> = HashMap::new();
    let mut symbol_coords = HashSet::new();
    let mut part_numbers: Vec<u32> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut running_num_str: String = String::from("");
        let mut running_num_start_coord: usize = usize::MAX;
        let mut running_num_end_coord: usize = usize::MAX;

        for (x, ch) in line.chars().enumerate() {
            match ch {
                '0'..='9' => {
                    if running_num_start_coord > x {
                        running_num_start_coord = x;
                    }
                    running_num_str.push(ch);
                    running_num_end_coord = x;
                }
                sym => {
                    if !running_num_str.is_empty() {
                        if let Some(number) = create_number(
                            &running_num_str,
                            running_num_start_coord,
                            running_num_end_coord,
                        ) {
                            number_coords.entry(y).or_default().push(number);
                        }
                        running_num_str.clear();
                        running_num_start_coord = usize::MAX;
                        running_num_end_coord = usize::MAX;
                    }
                    if sym != '.' {
                        symbol_coords.insert((x, y));
                    }
                }
            }
        }
        if !running_num_str.is_empty() {
            if let Some(number) = create_number(
                &running_num_str,
                running_num_start_coord,
                running_num_end_coord,
            ) {
                number_coords.entry(y).or_default().push(number);
            }
        }
    }

    for (x, y) in &symbol_coords {
        for adj_y in y.saturating_sub(1)..=y.saturating_add(1) {
            if let Some(row) = number_coords.get_mut(&adj_y) {
                row.retain(|num| {
                    let is_adjacent = ((&num.start <= x && x <= &num.end)
                        || (num.start <= x.saturating_sub(1) && x.saturating_sub(1) <= num.end)
                        || (num.start <= x.saturating_add(1) && x.saturating_add(1) <= num.end));

                    if is_adjacent {
                        part_numbers.push(num.value);
                        false
                    } else {
                        true
                    }
                })
            }
        }
    }

    part_numbers.iter().sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut number_coords: HashMap<usize, Vec<Number>> = HashMap::new();
    let mut gear_coords = HashSet::new();
    let mut gear_ratio: u32 = 0;

    for (y, line) in input.lines().enumerate() {
        let mut running_num_str: String = String::from("");
        let mut running_num_start_coord: usize = usize::MAX;
        let mut running_num_end_coord: usize = usize::MAX;

        for (x, ch) in line.chars().enumerate() {
            match ch {
                '0'..='9' => {
                    if running_num_start_coord > x {
                        running_num_start_coord = x;
                    }
                    running_num_str.push(ch);
                    running_num_end_coord = x;
                }
                sym => {
                    if !running_num_str.is_empty() {
                        if let Some(number) = create_number(
                            &running_num_str,
                            running_num_start_coord,
                            running_num_end_coord,
                        ) {
                            number_coords.entry(y).or_default().push(number);
                        }
                        running_num_str.clear();
                        running_num_start_coord = usize::MAX;
                        running_num_end_coord = usize::MAX;
                    }
                    if sym == '*' {
                        gear_coords.insert((x, y));
                    }
                }
            }
        }
        if !running_num_str.is_empty() {
            if let Some(number) = create_number(
                &running_num_str,
                running_num_start_coord,
                running_num_end_coord,
            ) {
                number_coords.entry(y).or_default().push(number);
            }
        }
    }

    for (x, y) in &gear_coords {
        let mut part_numbers: Vec<u32> = Vec::new();

        for adj_y in y.saturating_sub(1)..=y.saturating_add(1) {
            if let Some(row) = number_coords.get_mut(&adj_y) {
                for num in row {
                    let is_adjacent = ((&num.start <= x && x <= &num.end)
                        || (num.start <= x.saturating_sub(1) && x.saturating_sub(1) <= num.end)
                        || (num.start <= x.saturating_add(1) && x.saturating_add(1) <= num.end));

                    if is_adjacent {
                        part_numbers.push(num.value);
                    }
                }
            }
        }

        if part_numbers.len() == 2 {
            gear_ratio += part_numbers
                .iter()
                .copied()
                .reduce(|acc, e| acc * e)
                .unwrap();
        }
    }

    gear_ratio.into()
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
