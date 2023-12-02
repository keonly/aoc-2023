use lazy_regex::regex;
use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"[^\d]").unwrap();

    let ans: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = re
                .replace_all(line, "")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();

            match (digits.first(), digits.last()) {
                (Some(&first), Some(&last)) => first * 10 + last,
                _ => 0,
            }
        })
        .sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_f = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let re_b = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();

    let ans: u32 = input
        .lines()
        .map(|line| {
            let first: u32 = match re_f.find(line) {
                Some(m_f) => match m_f.as_str() {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    d => d.parse::<u32>().unwrap(),
                },
                _ => 0,
            };

            let last: u32 = match re_b.find(&*line.chars().rev().collect::<String>()) {
                Some(m_b) => match m_b.as_str() {
                    "eno" => 1,
                    "owt" => 2,
                    "eerht" => 3,
                    "ruof" => 4,
                    "evif" => 5,
                    "xis" => 6,
                    "neves" => 7,
                    "thgie" => 8,
                    "enin" => 9,
                    d => d.parse::<u32>().unwrap(),
                },
                _ => 0,
            };

            first * 10 + last
        })
        .sum();

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
