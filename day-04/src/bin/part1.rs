fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
	let mut collator = 0;

	for line in input.lines() {
		let numbers: Vec<&str> = line.split('|').collect();
		let winning_numbers: Vec<&str> = numbers[0].split(':').collect();
		let winning_numbers: Vec<&str> = winning_numbers[1].trim().split_whitespace().collect();
		let my_numbers: Vec<&str> = numbers[1].split_whitespace().collect();
		let mut line_collator = 0;
		for my_number in my_numbers {
			if winning_numbers.contains(&my_number) {
				println!("{:?} contains {}", winning_numbers, my_number);
				line_collator += 1;
			}
		}
		if line_collator > 0 {
			println!("score: {}", 2u32.pow(line_collator - 1));
			collator += 2u32.pow(line_collator - 1);
		}
	}

	return collator.to_string()
}