
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct RaceInfo {
    times_to_distances: Vec<(i64, i64)>,
}

#[aoc_generator(day6)]
pub fn construct_race_info(input: &str) -> RaceInfo {
    let times: Vec<i64> = input.lines().nth(0).unwrap().split_once(":").unwrap().1.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let distances: Vec<i64> = input.lines().nth(1).unwrap().split_once(":").unwrap().1.split_whitespace().map(|n| n.parse().unwrap()).collect();

    RaceInfo {
        times_to_distances: times.into_iter().zip(distances).collect()
    }
}

//use the quadratic forumula to solve for how long the button is held for the boat to achieve the same distance
#[aoc(day6, part1)]
pub fn p1(info: &RaceInfo) -> u32 {
    info.times_to_distances.iter().cloned().map(|(time, distance)| {
        let time = time as f32;
        let distance = distance as f32;

        let mut r1 = ((time - (time.powf(2.) - 4.*distance).sqrt()) / 2.).ceil() as u32;
        let mut r2 = ((time + (time.powf(2.) - 4.*distance).sqrt()) / 2.).floor() as u32;
        
        if (time as u32 - r1) * r1 <= distance as u32 { r1 += 1 }
        if (time as u32 - r2) * r2 <= distance as u32 { r2 -= 1 }

        r2 - r1 + 1
    })
    .reduce(|acc, e| acc * e)
    .unwrap()
}

#[aoc(day6, part2)]
pub fn p2(info: &RaceInfo) -> u64 {
    let times_joined = info.times_to_distances.iter().map(|(time, _)| format!("{}", time)).collect::<String>();
    let distances_joined = info.times_to_distances.iter().map(|(_, distance)| format!("{}", distance)).collect::<String>();

    let time = times_joined.parse::<u64>().unwrap() as f64;
    let distance = distances_joined.parse::<u64>().unwrap() as f64;

    let mut r1  = ((time - (time.powf(2.) - 4f64*distance).sqrt()) / 2.).ceil() as u64;
    let mut r2 = ((time + (time.powf(2.) - 4f64*distance).sqrt()) / 2.).floor() as u64;
    
    if (time as u64 - r1) * r1 <= distance as u64 { r1 += 1 }
    if (time as u64 - r2) * r2 <= distance as u64 { r2 -= 1 }

    r2 - r1 + 1
}

#[cfg(test)]
mod test {
    use super::*;


const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_part1() {
        let expected_output = 288;
        assert_eq!(expected_output, p1(&construct_race_info(INPUT)));
    }

    #[test]
    fn test_part2() {
        let expected_output = 71503;
        assert_eq!(expected_output, p2(&construct_race_info(INPUT)));
    }
}
