use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::iter::zip;

advent_of_code::solution!(6);

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, parse_u64)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, dists) = separated_list1(space1, parse_u64)(input)?;

    Ok((input, (times, dists)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (times, dists)) = parse_input(input).expect("a valid list");
    let matches: Vec<(u64, u64)> = zip(times, dists).collect();

    matches
        .iter()
        .filter_map(|(max_time, min_dist)| {
            let mut count: u64 = 0;
            for time in 0..=*max_time {
                if time * (*max_time - time) > *min_dist {
                    count = max_time + 1 - 2 * time;
                    break;
                }
            }

            if count > 0 {
                Some(count)
            } else {
                None
            }
        })
        .fold(1, |acc, count| acc * count)
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (times, dists)) = parse_input(input).expect("a valid list");
    let max_time = times
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let min_dist = dists
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let mut count: u64 = 0;

    for time in 0..=max_time {
        if time * (max_time - time) > min_dist {
            count = max_time + 1 - 2 * time;
            break;
        }
    }

    count.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
