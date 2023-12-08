use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;

fn main() {
    let test_input = include_str!("./input.txt");
    let output = part1(test_input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut collator = 0;

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut sequence;
    {
        let lines: Vec<&str> = input.lines().collect();
        sequence = lines[0].chars().collect::<VecDeque<char>>();
        for (i, line) in input.lines().enumerate() {
            match i {
                2.. => {
                    let re_value = Regex::new(r"[A-Z]{3}").unwrap();
                    let values:Vec<_> = re_value.find_iter(line).collect();
                    map.insert(values[0].as_str().to_string(), (values[1].as_str().to_string(), values[2].as_str().to_string()));
                }
                _ => ()
            }
        }
    }

    let mut location:String = "AAA".to_string();
    while location != "ZZZ".to_string() {
        match sequence.front().unwrap() {
            'L' => location = map.get(&location).unwrap().0.clone(),
            'R' => location = map.get(&location).unwrap().1.clone(),
            _ => panic!()
        }
        sequence.rotate_left(1);
        collator += 1;
    }

    collator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let result = part1(include_str!("./test_input.txt"));
        assert_eq!(result, "6");
    }
}