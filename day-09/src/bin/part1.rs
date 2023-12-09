fn main() {
    let test_input = include_str!("./input.txt");
    let output = part1(test_input);
    dbg!(output);
}

fn extrapolate(sequence:Vec<u64>) -> u64 {
    if sequence.iter().fold(true, |acc, x| acc && *x == 0u64) {
        return 0
    } else {
        // TODO solution
        return extrapolate(   //sequence.fold(Vec::from([sequence[0]]), |acc: Vec<u64>, x| (x - acc.0, acc.1.push(x - acc.0)).1))
    }
}

fn part1(input: &str) -> String {
    let mut collator = 0;
    for line in input.lines() {
        let seq:Vec<u64> = line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
        collator += extrapolate(seq);
    }
    collator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let result = part1(include_str!("./test_input.txt"));
        assert_eq!(result, "114");
    }
}