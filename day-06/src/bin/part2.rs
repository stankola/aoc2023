fn main() {
    let test_input = include_str!("./input.txt");
    let output = part2(test_input);
    dbg!(output);
}

struct Game{
    time: u64,
    distance: u64
}

fn parse_lines(input: &str) -> Game {
    let time: String;
    let distance: String;

    let lines:Vec<&str> = input.lines().collect();
    time = lines[0].split(':').collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>().join("");
    distance = lines[1].split(':').collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>().join("");
    Game{time: time.parse::<u64>().unwrap(), distance:distance.parse::<u64>().unwrap()}
}

fn part2(input: &str) -> String {
    let mut collator = 0;

    let game = parse_lines(input);

    for t in 0..game.time {
        let dist = t * (game.time - t);
        if dist > game.distance {
            collator += 1;
        }
    }

    collator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let result = part2(include_str!("./test_input.txt"));
        assert_eq!(result, "71503");
    }
}
