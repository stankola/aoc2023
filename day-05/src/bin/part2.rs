use std::{ops::Range, collections::VecDeque};
use regex::Regex;

fn main() {
    let test_input = include_str!("./input.txt");
    let output = part2(test_input);
    dbg!(output);
}

fn add_seeds(vec: &mut Vec<Range<u64>>, input: &str) {
    let re_seed_range = Regex::new(r"\d+ \d+").unwrap();
    for input_range in re_seed_range.find_iter(input) {
        let range:Vec<&str> = input[input_range.start()..input_range.end()].split_whitespace().collect();
        vec.push(Range{
            start: range[0].parse::<u64>().unwrap(), 
            end: range[0].parse::<u64>().unwrap() + range[1].parse::<u64>().unwrap()
        });
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

// There's probably a way to make this generic
fn range_contains_range(a: &Range<u64>, b: &Range<u64>) -> bool {
    return a.start >= b.start && a.end <= b.end
}

fn part2(input: &str) -> String {
    let mut seeds:Vec<Range<u64>> = vec![];
    let mut maps:Vec<Vec<(Range<u64>, Range<u64>)>> = vec![];
    let mut map_i = -1;

    let re_seeds = Regex::new(r"seeds").unwrap();
    let re_map = Regex::new(r".*map").unwrap();
    let re_ranges = Regex::new(r"\d+ \d+ \d+").unwrap();

    for line in input.lines() {
        let split: Vec<&str> = line.split(':').collect();
        match split[0] {
            t if re_seeds.is_match(t) => add_seeds(&mut seeds, split[1].trim()),
            t if re_map.is_match(t) => {maps.push(vec![]); map_i += 1},
            t if re_ranges.is_match(t) => maps[map_i as usize].push(parse_ranges(split[0])),
            _ => ()
        }
    }
    let mut lowest_loc = u64::MAX;
    for seed_range in seeds {
        println!("processing range {}-{}: {} values", seed_range.start, seed_range.end, seed_range.end - seed_range.start);
        let mut seed_start_ranges:VecDeque<Range<u64>> = VecDeque::new();
        let mut seed_target_ranges:VecDeque<Range<u64>> = VecDeque::new();
        seed_start_ranges.push_back(seed_range);

        for map in &maps {
            println!("seed_start_ranges @ loopstart {:?}", seed_start_ranges);
            while !seed_start_ranges.is_empty() {
                let mut seed_start_range = seed_start_ranges.pop_front().unwrap();
                println!("processing queue {}-{}: {} values", seed_start_range.start, seed_start_range.end, seed_start_range.end - seed_start_range.start);
                for ranges in map {
                    println!("Ranges {}-{} fits in {}-{}", seed_start_range.start, seed_start_range.end, ranges.0.start, ranges.0.end);
                    if ranges.0.contains(&seed_start_range.start) { // TODO other cases: ranges.0.contains(&seed_start_range.end) AND seed_start_range.contains(ranges.0)
                        if ranges.0.contains(&seed_start_range.end) {
                            println!("Perfectly");
                            let target_ranges: &mut VecDeque<Range<u64>> = &mut seed_target_ranges;
                            // range fits completely. Push target range and move to next start range
                            let len = seed_start_range.end - seed_start_range.start;
                            let offset = seed_start_range.start - ranges.0.start;
                            target_ranges.push_back(Range{ start: ranges.1.start + offset,
                                                                end: ranges.1.start + offset + len});
                            seed_start_range.start = 0;
                            seed_start_range.end = 0;
                            break;
                        }
                        else {
                            // range fits partially. Push fitting range and shorten start range
                            // TODO index check. Done.
                            println!("Not so perfectly");
                            let target_ranges: &mut VecDeque<Range<u64>> = &mut seed_target_ranges;
                            let len = ranges.0.end - seed_start_range.start - 1;
                            let offset = seed_start_range.start - ranges.0.start;
                            target_ranges.push_back(Range{start: ranges.1.start + offset,
                                end: ranges.1.start + offset + len});
                            seed_start_range.start = seed_start_range.start + len;
                        }
                    }
                    else if ranges.0.contains(&seed_start_range.end){
                        // TODO
                    }
                    else if range_contains_range(&seed_start_range, &ranges.0) {
                        // TODO
                    }
                    else {
                        println!("Not at all");
                    }
                }
                if !seed_start_range.is_empty() {
                    seed_target_ranges.push_back(seed_start_range);
                }
            }
            seed_start_ranges = seed_target_ranges.clone();     // Is this a shallow copy...?
            seed_target_ranges.clear();                 // Does this affect seed_start_ranges?
            println!("target start after {:?}", seed_start_ranges);
        }
        for target in seed_target_ranges {
            if target.start < lowest_loc {
                lowest_loc = target.start;
            }
        }
   }

/* // Unoptimized:
   for seed_range in seeds {
   println!("processing range {}-{}: {} values", seed_range.start, seed_range.end, seed_range.end - seed_range.start);
        for seed in seed_range {
            let mut loc = seed;
            for map in &maps {
                for ranges in map {
                    if ranges.0.contains(&loc) {
                        loc = ranges.1.start + loc - ranges.0.start;
                        break;
                    }
                }
            }
            lowest_loc = if loc < lowest_loc {loc} else {lowest_loc};
        }
    }
*/

    lowest_loc.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let result = part2(include_str!("./test_input.txt"));
        assert_eq!(result, "46");
    }
}