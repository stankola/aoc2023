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
    return a.start <= b.start && a.end >= b.end
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
        let mut start_queue:VecDeque<Range<u64>> = VecDeque::new();
        let mut target_queue:VecDeque<Range<u64>> = VecDeque::new();
        start_queue.push_back(seed_range);

        for map in &maps {
            'start_loop: while !start_queue.is_empty() {
                let mut input_seeds = start_queue.pop_front().unwrap();
                for filter in map {
                    if filter.0.contains(&input_seeds.start) {
                        if filter.0.contains(&(input_seeds.end - 1)) {
                            // Seed range fits completely inside the map range
                            let len = input_seeds.end - input_seeds.start;
                            let offset = input_seeds.start - filter.0.start;
                            target_queue.push_back(Range{   start: filter.1.start + offset,
                                                            end: filter.1.start + offset + len});
                            break 'start_loop;
                        }
                        else {
                            // Start of seed range fits inside the map range
                            let len = filter.0.end - input_seeds.start;
                            let offset = input_seeds.start - filter.0.start;
                            target_queue.push_back(Range{   start: filter.1.start + offset,
                                                            end: filter.1.start + offset + len});
                            input_seeds.start = input_seeds.start + len;
                        }
                    }
                    else if filter.0.contains(&(input_seeds.end - 1)){
                        // End of seed range fits inside the map range
                        let len = input_seeds.end - filter.0.start;
                        target_queue.push_back(Range{   start: filter.1.start,
                                                        end: filter.1.start + len});
                        input_seeds.end = input_seeds.end - len;
                    }
                    else if range_contains_range(&input_seeds, &filter.0) {
                        // The map range is between the start and end of seed range
                        target_queue.push_back(Range{   start: filter.1.start,
                                                        end: filter.1.end});
                        start_queue.push_back(Range{start: filter.0.end,
                                                    end: input_seeds.end});
                        input_seeds.end = filter.0.start;
                    }
                }
                // If no mapping was found for some groups of seeds, move them forward as they are, as per subject
                target_queue.push_back(input_seeds);
            }
            // Make target_queue the start_queue for the next set of filters
            start_queue = target_queue.clone();
            target_queue.clear();
        }
        for target in start_queue {
            if target.start < lowest_loc {
                lowest_loc = target.start;
            }
        }
    }
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