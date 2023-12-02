fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Copy, Clone, Debug)]
struct Set {
    r: u32,
    g: u32,
    b: u32,
}

struct Game {
    number: u32,
    sets: Vec<Set>,
}

// Pretty naive code but it does the trick
fn parse_line(line: &str) -> Game {
    let mut game = Game {
        number: 0,
        sets: Vec::new(),
    };

    let mut p = line.split(':');
    game.number = p.next().unwrap().strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
    p = p.next().unwrap().split(';');
    for set_input in p {
        let mut set = Set {r:0, g:0, b:0};
        for color in set_input.split(',') {
            let splitted_col:Vec<&str> = color.split(' ').collect();
            if splitted_col[2].eq("red") {
                set.r = splitted_col[1].parse::<u32>().unwrap();
            }
            else if splitted_col[2].eq("green") {
                set.g = splitted_col[1].parse::<u32>().unwrap();
            }
            else if splitted_col[2].eq("blue") {
                set.b = splitted_col[1].parse::<u32>().unwrap();
            }
        }
        game.sets.push(set);
    }
    game
}

fn part2(input: &str) -> String {
    let mut collator = 0;

    'main: for line in input.lines() {
        let game: Game = parse_line(line);
        println!("game {}: {:?}", game.number, game.sets);
		let mut max_r = 0;
		let mut max_g = 0;
		let mut max_b = 0;
        for set in game.sets {
			max_r = if set.r > max_r {set.r} else {max_r};
			max_g = if set.g > max_g {set.g} else {max_g};
			max_b = if set.b > max_b {set.b} else {max_b};
        }
        collator += max_r * max_g * max_b;
    }
    collator.to_string()
}