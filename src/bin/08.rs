use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace1, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(8);

fn parse_control_seq(input: &str) -> IResult<&str, &str> {
    let (input, control_sequence): (&str, &str) = terminated(alpha1, multispace1)(input)?;
    Ok((input, control_sequence))
}

fn parse_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, key) = terminated(alpha1, space1)(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace1(input)?;
    let mut value_parser = delimited(
        char('('),
        separated_pair(alpha1, tag(", "), alpha1),
        char(')'),
    );
    let (input, value) = value_parser(input)?;

    Ok((input, (key, value)))
}

fn parse_input(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let (input, lines) = separated_list1(multispace1, parse_line)(input)?;
    Ok((input, lines.into_iter().collect()))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input, control_sequence_string) = parse_control_seq(input).expect("a valid input");
    let (_, instruction_map) = parse_input(input).expect("a valid input");
    let mut step_count = 0;
    let mut curr: &str = "AAA";
    let mut control_sequence: VecDeque<char> = control_sequence_string.chars().collect();

    while curr != "ZZZ" {
        let current_control = control_sequence.pop_front().unwrap();
        control_sequence.push_back(current_control);

        match current_control {
            'L' => {
                curr = instruction_map.get(curr).unwrap().0;
            }
            'R' => {
                curr = instruction_map.get(curr).unwrap().1;
            }
            c => {
                panic!("Unexpected control character: {}", c);
            }
        }
        step_count += 1;
    }

    Some(step_count)
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    let gcd = gcd(a, b);
    if gcd != 0 {
        (a / gcd) * b
    } else {
        a * b
    }
}

pub fn part_two(input: &str) -> Option<u128> {
    let (input, control_sequence_string) = parse_control_seq(input).expect("a valid input");
    let (_, instruction_map) = parse_input(input).expect("a valid input");
    let mut step_counts_vec: Vec<u128> = vec![];

    for (key, _) in instruction_map.iter() {
        if key.ends_with("A") {
            let mut curr: &str = key;
            let mut control_sequence: VecDeque<char> = control_sequence_string.chars().collect();
            let mut step_count = 0;

            while !curr.ends_with("Z") {
                let current_control = control_sequence.pop_front().unwrap();
                control_sequence.push_back(current_control);

                match current_control {
                    'L' => {
                        curr = instruction_map.get(curr).unwrap().0;
                    }
                    'R' => {
                        curr = instruction_map.get(curr).unwrap().1;
                    }
                    c => {
                        panic!("Unexpected control character: {}", c);
                    }
                }
                step_count += 1;
            }

            step_counts_vec.push(step_count);
        }
    }

    let total_count = step_counts_vec.iter().fold(1, |acc, val| lcm(acc, *val));
    Some(total_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
