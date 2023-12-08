use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;

fn main() {
    let test_input = include_str!("./input.txt");
    let output = part2(test_input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut collator = 0;

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let sequence;
    let mut ghost_locations: Vec<String> = vec![];
    {
        let lines: Vec<&str> = input.lines().collect();
        sequence = lines[0].chars().collect::<VecDeque<char>>();
        for (i, line) in input.lines().enumerate() {
            match i {
                2.. => {
                    let re_value = Regex::new(r"\w{3}").unwrap();
                    let values:Vec<_> = re_value.find_iter(line).collect();
                    map.insert(values[0].as_str().to_string(), (values[1].as_str().to_string(), values[2].as_str().to_string()));
                    if values[0].as_str().ends_with('A') {
                        ghost_locations.push(values[0].as_str().to_string());
                    }
                }
                _ => ()
            }
        }
    }

    let mut ghost_path_lengths:Vec<u64> = vec![];
    for mut ghost in ghost_locations {
        let mut this_sequence = sequence.clone();
        let mut distance = 0;
        while !ghost.ends_with('Z') {
            distance += 1;
            match this_sequence.front().unwrap() {
                'L' =>  {
                            ghost.replace_range(0.., &map.get(&ghost).unwrap().0);
                        },
                'R' =>  {
                            ghost.replace_range(0.., &map.get(&ghost).unwrap().1);
                        },
                _ => panic!()
            }
            this_sequence.rotate_left(1);
        }
        ghost_path_lengths.push(distance);
    }
/*
[src/bin/part2.rs:66] ghost_path_lengths = [
    18157,
    14363,
    16531,
    12737,
    19783,
    19241,
] */

    // find i that is divisible by each length.
    let min_length = *ghost_path_lengths.iter().min().unwrap();
    'i_loop: for i in 1.. {
        for length in &ghost_path_lengths {
            if (i * min_length) % length != 0 {
                continue 'i_loop;
            }
        }
        collator = i * min_length;
        break;
    }

    collator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let result = part2(include_str!("./test_input2.txt"));
        assert_eq!(result, "6");
    }
}