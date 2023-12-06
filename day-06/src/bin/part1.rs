fn main() {
    let test_input = include_str!("./input.txt");
    let output = part1(test_input);
    dbg!(output);
}

struct Game{
    time: u32,
    distance: u32
}

fn parse_lines(input: &str) -> Vec<Game> {
    let times: Vec<&str>;
    let distances: Vec<&str>;
    let mut games: Vec<Game> = vec![];

    let lines:Vec<&str> = input.lines().collect();
    times = lines[0].split(':').collect::<Vec<&str>>()[1].trim().split_whitespace().collect();
    distances = lines[1].split(':').collect::<Vec<&str>>()[1].trim().split_whitespace().collect();
    for (i, time) in times.iter().enumerate() {
        let distance = distances[i];
        games.push(Game{time: time.parse::<u32>().unwrap(), distance:distance.parse::<u32>().unwrap()});
    }
    games
}

fn part1(input: &str) -> String {
    let mut collator = 1;

    let games: Vec<Game> = parse_lines(input);

    for game in games {
        let mut game_collator = 0;
        for t in 0..game.time {
            let dist = t * (game.time - t);
            if dist > game.distance {
                game_collator += 1;
            }
        }
        collator *= game_collator;
    }

    collator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let result = part1(include_str!("./test_input.txt"));
        assert_eq!(result, "288");
    }
}
