
use std::{ops::Range, cmp::{max, min}, iter};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct PlantingInfo {
    seeds: Vec<i64>,
    conversions: Vec<Vec<(Range<i64>, i64)>>
}

#[aoc_generator(day5)]
pub fn construct_planting_info(input: &str) -> PlantingInfo {
    let (seeds, rest) = input.split_once("\n").unwrap();
    
    let seeds: Vec<i64> = seeds
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();
    
    let conversions = 
        rest.trim().split("\n\n").map(|section| {
            section.lines()
                .skip(1)
                .map(|l| -> Vec<i64> { l.split(' ').map(|n| n.parse().unwrap()).take(3).collect() })
                .map(|v| (v[1]..v[1]+v[2], v[0] - v[1]))
                .collect()
        })
        .collect();

    let info = PlantingInfo {
        seeds,
        conversions
    };

    info
}

#[aoc(day5, part1)]
pub fn p1(info: &PlantingInfo) -> u32 {
    info.seeds.iter().map(|seed| {
        let mut current_value = *seed;

        for conversion in info.conversions.iter() {
            if let Some((_, mapping)) = conversion.iter().find(|(range, _)| range.contains(&current_value)) {
                current_value = current_value + mapping; 
            }
        }

        current_value
    })
    .min()
    .unwrap() as u32
}

#[aoc(day5, part2)]
pub fn p2(info: &PlantingInfo) -> u32 {
    let ranges_intersect = |r1: &Range<i64>, r2: &Range<i64>| r1.start <= r2.end && r2.start <= r1.end;
    let intersect_range = |r1: &Range<i64>, r2: &Range<i64>| if ranges_intersect(r1, r2) { Some(max(r1.start, r2.start)..min(r1.end, r2.end)) } else {None }; 

    let ranges_equal = |r1: &Range<i64>, r2: &Range<i64>| r1.start == r2.start && r1.end == r2.end;

    let seed_ranges: Vec<Range<i64>> = info.seeds.chunks(2).map(|s| (s[0]..s[0]+s[1])).collect();

    seed_ranges.into_iter().map(|seed| {
        let mut ranges = vec![seed];
        
        for conversion in info.conversions.iter() {
            let mut new_ranges: Vec<Range<i64>> = Vec::new();

            for range in ranges.iter() {
                let mut intersected_ranges: Vec<(Range<i64>, i64)> = conversion
                    .iter()
                    .filter_map(|(conversion_range, off)| intersect_range(range, conversion_range).map(|inter| (inter, *off)))
                    .collect();
                
                intersected_ranges.sort_by_key(|(range, _)| range.start);

                if intersected_ranges.len() == 0 { new_ranges.push(range.clone()); continue }                   

                let mut input_range_pois: Vec<i64> = vec![range.start];
                input_range_pois.extend(intersected_ranges
                    .iter()
                    .map(|(r, _)| [r.start, r.end])
                    .flatten()
                );
                input_range_pois.push(range.end);
                input_range_pois.dedup();

                new_ranges.extend(
                    input_range_pois
                        .windows(2)
                        .map(|w| {
                            let intersected_range = intersected_ranges
                                .iter()
                                .find(|(r, _)| ranges_equal(r, &(w[0]..w[1])));

                            if let Some((_, offset)) = intersected_range {
                                w[0]+offset..w[1]+offset
                            } else {
                                w[0]..w[1]
                            }
                        })
                )
            }

            ranges = new_ranges;
        }

        ranges.iter().map(|range| range.start).min().unwrap()
    })
    .min()
    .unwrap() as u32
}

#[cfg(test)]
mod test {
    use super::*;


const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_part1() {
        let expected_output = 35;
        assert_eq!(expected_output, p1(&construct_planting_info(INPUT)));
    }

    #[test]
    fn test_part2() {
        let expected_output = 46;
        assert_eq!(expected_output, p2(&construct_planting_info(INPUT)));
    }
}
