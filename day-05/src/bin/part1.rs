use std::ops::Range;
use regex::Regex;

fn main() {
    let test_input = include_str!("./test_input.txt");
    let output = part1(test_input);
    dbg!(output);
}

fn add_seeds(vec: &mut Vec<u64>, seed_input: &str) {
    let seeds: Vec<&str> = seed_input.trim().split_whitespace().collect();
    for seed in seeds {
        vec.push(seed.parse::<u64>().unwrap());
    }
}

fn parse_ranges(input: &str) -> (Range<u64>, Range<u64>) {
    let from_start:u64;
    let from_end:u64;
    let to_start:u64;
    let to_end:u64;

    let arguments:Vec<&str> = input.split_whitespace().collect();
    to_start = arguments[0].parse::<u64>().unwrap();
    from_start = arguments[1].parse::<u64>().unwrap();
    from_end = from_start + arguments[2].parse::<u64>().unwrap();
    to_end = to_start + arguments[2].parse::<u64>().unwrap();

    (Range{start:from_start, end:from_end}, Range{start:to_start, end:to_end})
}

fn part1(input: &str) -> String {
    let mut seeds:Vec<u64> = vec![];
    let mut maps:Vec<Vec<(Range<u64>, Range<u64>)>> = vec![];
    let mut map_i = -1;

    let re_seeds = Regex::new(r"seeds").unwrap();
    let re_map = Regex::new(r".*map").unwrap();
    let re_ranges = Regex::new(r"\d+ \d+ \d+").unwrap();

    for line in input.lines() {
        let split: Vec<&str> = line.split(':').collect();
        match split[0] {
            t if re_seeds.is_match(t) => add_seeds(&mut seeds, split[1]),
            t if re_map.is_match(t) => {maps.push(vec![]); map_i += 1},
            t if re_ranges.is_match(t) => maps[map_i as usize].push(parse_ranges(split[0])),
            _ => ()
        }
    }
    let mut lowest_loc = u64::MAX;
    for seed in seeds {
        println!("seed {seed}");
        let mut loc = seed;
        for map in &maps {
            for ranges in map {
                if ranges.0.contains(&loc) {
                    loc = ranges.1.start + loc - ranges.0.start;
                    println!("to {loc}");
                    break;
                }
                // If ranges do not contain loc, it should pass through as it is. I did not notice this requirement initially and coded it in by accident. Happy little coincidence!
            }
        }
        lowest_loc = if loc < lowest_loc {loc} else {lowest_loc};
   }

    lowest_loc.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let result = part1(include_str!("./test_input.txt"));
        assert_eq!(result, "35");
    }
}