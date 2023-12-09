fn main() {
    let test_input = include_str!("./input.txt");
    let output = part2(test_input);
    dbg!(output);
}

fn extrapolate(sequence:&Vec<i64>) -> i64 {
    if sequence.iter().fold(true, |acc, x| acc && *x == 0i64) {
        return 0
    }
    let mut sequence_2: Vec<i64> = vec![];
    for v in sequence.iter().enumerate() {
        if v.0 == 0 {continue;}
        sequence_2.push(*(v.1) - sequence[v.0 - 1]);
    }
    return sequence_2.last().unwrap() + extrapolate(&sequence_2);
}

fn part2(input: &str) -> String {
    let mut collator = 0;
    for line in input.lines() {
        let mut seq:Vec<i64> = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
		seq.reverse();
        collator += seq.last().unwrap() + extrapolate(&seq);
    }
    collator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let result = part2(include_str!("./test_input.txt"));
        assert_eq!(result, "2");
    }
}