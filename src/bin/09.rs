advent_of_code::solution!(9);

fn parse_line(line: &str) -> Vec<i32> {
    line.trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .map(Result::unwrap)
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut next_elems: Vec<i32> = vec![];
    for line in input.lines() {
        let vec_line = parse_line(line);
        let mut vec_diffs: Vec<Vec<i32>> = vec![vec_line];

        loop {
            let diff: Vec<i32> = vec_diffs
                .last()
                .unwrap()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect();
            vec_diffs.push(diff);
            if vec_diffs.last().unwrap().iter().all(|x| *x == 0) {
                break;
            }
        }

        let next_elem: i32 = vec_diffs.iter().map(|v| v.last().unwrap()).sum();
        next_elems.push(next_elem);
    }

    next_elems.iter().sum::<i32>().into()
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut next_elems: Vec<i32> = vec![];
    for line in input.lines() {
        let mut vec_line = parse_line(line);
        vec_line.reverse();
        let mut vec_diffs: Vec<Vec<i32>> = vec![vec_line];

        loop {
            let diff: Vec<i32> = vec_diffs
                .last()
                .unwrap()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect();
            vec_diffs.push(diff);
            if vec_diffs.last().unwrap().iter().all(|x| *x == 0) {
                break;
            }
        }

        let next_elem: i32 = vec_diffs.iter().map(|v| v.last().unwrap()).sum();
        next_elems.push(next_elem);
    }

    next_elems.iter().sum::<i32>().into()
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
