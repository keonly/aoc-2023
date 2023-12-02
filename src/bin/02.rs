use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let game_re = Regex::new(r"Game (\d+): (.+)").unwrap();
    let cube_re = Regex::new(r"(\d+) (\w+)").unwrap();
    let cube_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .lines()
        .filter_map(|line| {
            let caps = game_re.captures(line)?;
            let game_id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let sets_str = caps.get(2).unwrap().as_str();

            let valid_game = sets_str.split(";").all(|set| {
                set.split(",").all(|entry| {
                    if let Some(cap) = cube_re.captures(entry) {
                        let count = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                        let colour = cap.get(2).unwrap().as_str();
                        cube_limits
                            .get(colour)
                            .map_or(false, |&limit| count <= limit)
                    } else {
                        false
                    }
                })
            });

            if valid_game {
                Some(game_id)
            } else {
                None
            }
        })
        .sum::<u32>()
        .into()
}

#[derive(Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn update_max(&mut self, colour: &str, count: u32) {
        match colour {
            "red" => self.red = self.red.max(count),
            "green" => self.green = self.green.max(count),
            "blue" => self.blue = self.blue.max(count),
            _ => (),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let game_re = Regex::new(r"Game (\d+): (.+)").unwrap();
    let cube_re = Regex::new(r"(\d+) (\w+)").unwrap();
    let split_re = Regex::new(r"[,;]\s*").unwrap();

    input
        .lines()
        .filter_map(|line| {
            let sets_str = game_re.captures(line).unwrap().get(2).unwrap().as_str();

            let max_cubes: Cubes =
                split_re
                    .split(sets_str)
                    .fold(Cubes::default(), |mut acc, cube_desc| {
                        if let Some(cap) = cube_re.captures(cube_desc) {
                            let count = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                            let colour = cap.get(2).unwrap().as_str();

                            acc.update_max(colour, count);
                        }
                        acc
                    });

            Some(max_cubes.red * max_cubes.green * max_cubes.blue)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
