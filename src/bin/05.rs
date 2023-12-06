use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::cmp::{max, min};
use std::ops::Range;

advent_of_code::solution!(5);

#[derive(Debug)]
struct RangedSet(Vec<Range<u64>>);

impl RangedSet {
    pub fn iter(&self) -> impl Iterator<Item = &Range<u64>> {
        self.0.iter()
    }

    pub fn range_intersection(range1: &Range<u64>, range2: &Range<u64>) -> Option<Range<u64>> {
        let intersection_start = max(range1.start, range2.start);
        let intersection_end = min(range1.end, range2.end);

        if intersection_start < intersection_end {
            Some(intersection_start..intersection_end)
        } else {
            None
        }
    }

    pub fn range_complement(
        bigger_range: &Range<u64>,
        smaller_range: &Range<u64>,
    ) -> Vec<Range<u64>> {
        let mut complements: Vec<Range<u64>> = vec![];

        if bigger_range.start < smaller_range.start {
            complements.push(bigger_range.start..smaller_range.start);
        }
        if bigger_range.end > smaller_range.end {
            complements.push(smaller_range.end..bigger_range.end);
        }

        complements
    }

    fn remove_from_set(&mut self, range: &Range<u64>) {
        let index_and_intersection: Vec<(usize, Range<u64>, Range<u64>)> = self
            .iter()
            .enumerate()
            .filter_map(|(i, r)| {
                if let Some(intersection) = Self::range_intersection(r, range) {
                    Some((i, r.start..r.end, intersection))
                } else {
                    None
                }
            })
            .collect();

        let (idx, original_range, intersecting_range) = &index_and_intersection[0];

        assert!(
            original_range.start <= intersecting_range.start
                && intersecting_range.end <= original_range.end,
            "Original range is {}..{}, Intersecting range is {}..{}",
            original_range.start,
            original_range.end,
            intersecting_range.start,
            intersecting_range.end
        );

        self.0.remove(*idx);

        self.0
            .extend(Self::range_complement(&original_range, &intersecting_range));
    }
}

type Seeds = Vec<u64>;
type AlmanacMapping = Vec<(Range<u64>, Range<u64>)>;
struct Almanac(Vec<AlmanacMapping>);

impl Almanac {
    pub fn iter(&self) -> impl Iterator<Item = &AlmanacMapping> {
        self.0.iter()
    }

    fn translate_mapping(&self, seed: u64) -> u64 {
        self.iter().fold(seed, |src_acc, mapping| {
            let found_single_mapping = mapping
                .iter()
                .find(|(src_range, _)| src_range.contains(&src_acc));
            match found_single_mapping {
                Some((src_range, dest_range)) => dest_range.start + (src_acc - src_range.start),
                None => src_acc,
            }
        })
    }

    fn translate_ranged_mapping(&self, seed_range: Range<u64>) -> Vec<Range<u64>> {
        self.iter()
            .fold(vec![seed_range], |src_range_acc, mapping| {
                let mut translated_range: Vec<Range<u64>> = vec![];

                for src_range in src_range_acc {
                    let mut range_set = RangedSet(vec![src_range.start..src_range.end]);
                    for (map_src_range, map_dest_range) in mapping {
                        if let Some(intersection) =
                            RangedSet::range_intersection(&src_range, &map_src_range)
                        {
                            translated_range.push(
                                (intersection.start + map_dest_range.start - map_src_range.start)
                                    ..(intersection.end + map_dest_range.start
                                        - map_src_range.start),
                            );
                            range_set.remove_from_set(&intersection);
                        }
                    }
                    translated_range.extend(range_set.0);
                }

                translated_range
            })
    }
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Seeds> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(space1, parse_u64))(input)?;
    Ok((input, seeds))
}

fn parse_single_mapping(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (dest_start, _, src_start, _, length)) =
        tuple((parse_u64, space1, parse_u64, space1, parse_u64))(input)?;
    let src_range = src_start..(src_start + length);
    let dest_range = dest_start..(dest_start + length);
    Ok((input, (src_range, dest_range)))
}

fn parse_almanac_mapping(input: &str) -> IResult<&str, AlmanacMapping> {
    let (input, mappings) = many1(terminated(parse_single_mapping, line_ending))(input)?;
    Ok((input, mappings))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let mapping_names = vec![
        "seed-to-soil map:\n",
        "soil-to-fertilizer map:\n",
        "fertilizer-to-water map:\n",
        "water-to-light map:\n",
        "light-to-temperature map:\n",
        "temperature-to-humidity map:\n",
        "humidity-to-location map:\n",
    ];
    let mut mappings_vector: Vec<AlmanacMapping> = vec![];
    let mut input = input;

    for name in mapping_names {
        let parse_result = take_until(name)(input)?;
        input = parse_result.0;
        let parse_result = tag(name)(input)?;
        input = parse_result.0;
        let parse_result = parse_almanac_mapping(input)?;
        input = parse_result.0;
        mappings_vector.push(parse_result.1);
    }

    Ok((input, Almanac(mappings_vector)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (input, seeds) = parse_seeds(input).expect("a valid seeds list");
    let (_, almanac) = parse_almanac(input).expect("a valid mapping");

    seeds
        .into_iter()
        .map(|seed| almanac.translate_mapping(seed))
        .min()
        .into()
}

type RangedSeeds = Vec<Range<u64>>;

fn parse_single_ranged_seed(input: &str) -> IResult<&str, Range<u64>> {
    let (input, (start, _, length)) = tuple((parse_u64, space1, parse_u64))(input)?;
    Ok((input, start..(start + length)))
}

fn parse_ranged_seeds(input: &str) -> IResult<&str, RangedSeeds> {
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list1(space1, parse_single_ranged_seed),
    )(input)?;
    Ok((input, seeds))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (input, seeds) = parse_ranged_seeds(input).expect("a valid seeds list");
    let (_, almanac) = parse_almanac(input).expect("a valid mapping");

    let mut translated_ranges: Vec<Range<u64>> = vec![];

    for seed in seeds {
        let location = almanac.translate_ranged_mapping(seed);
        translated_ranges.extend(location);
    }

    translated_ranges
        .iter()
        .map(|range| range.start)
        .min()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
