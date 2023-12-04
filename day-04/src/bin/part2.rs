use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
	let mut collator = 0;

	let mut queue: VecDeque<&str> = VecDeque::new();
	let lines: Vec<&str> = input.lines().collect();
	for line in &lines {
		queue.push_back(line);
	}

	while !queue.is_empty() {
		let line = queue.pop_front().unwrap();
		let matches = get_matches(line);
		let card_no = get_card_number(line) + 1;
		for copy_card in card_no..(card_no + matches) {
			queue.push_back(&lines[(copy_card - 1) as usize]);
		}
		collator += 1;
	}

	return collator.to_string()
}

fn get_matches(line: &str) -> u32 {
    let numbers: Vec<&str> = line.split('|').collect();
    let winning_numbers: Vec<&str> = numbers[0].split(':').collect();
    let winning_numbers: Vec<&str> = winning_numbers[1].trim().split_whitespace().collect();
    let my_numbers: Vec<&str> = numbers[1].split_whitespace().collect();
    let mut matches = 0;
    for my_number in my_numbers {
		if winning_numbers.contains(&my_number) {
			matches += 1;
		}
	}
	matches
}

fn get_card_number(line: &str) -> u32 {
	let prefix:&str = line.split(':').collect::<Vec<&str>>()[0];
	prefix.rsplit(' ').collect::<Vec<&str>>()[0].parse::<u32>().unwrap()
}