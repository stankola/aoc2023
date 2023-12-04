fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Value {
	num: u32,
	pos: Point,
	len: usize
}

struct Point {
	x: usize,
	y: usize
}

fn scan_neighbours(val: &Value, grid: &Vec::<Vec<char>>) -> bool {
	use std::ops::Range;

	println!("{} y:{} x:{} len:{}", val.num, val.pos.y, val.pos.x, val.len);

	let xrange = {
		let end = if val.pos.x + val.len + 1 >= grid[0].len() {
			val.pos.x + val.len
		} else {
			val.pos.x + val.len + 1
		};
		let start = if val.pos.x != 0 {
			val.pos.x - 1
		} else {
			val.pos.x
		};
		Range{start, end}
	};
	
	let yrange = {
		let end = if val.pos.y + 1 == grid.len() {
			val.pos.y + 1
		} else {
			val.pos.y + 2
		};
		let start = if val.pos.y != 0 {
			val.pos.y - 1
		} else {
			val.pos.y
		};
		Range{start, end}
	};

	for y in yrange {
		for x in xrange.clone() {
			if y == val.pos.y && x >= val.pos.x && x < val.pos.x + val.len {
				continue;
			}
			println!("checking y:{} x:{}", y, x);
			if grid[y][x] != '.' && !grid[y][x].is_digit(10) {
				println!("Found char {} at y:{} x:{}", grid[y][x], y, x);
				return true
			}
		}
	}
	false
}


fn part1(input: &str) -> String {
	let mut collator = 0;

	let mut grid: Vec::<Vec<char>> = vec![];
	let mut values: Vec::<Value> = vec![];

	let mut i = 0;
	for line in input.lines() {
		grid.push(line.chars().collect::<Vec<char>>());
		let mut value_collator = (0, 0, 0);
		for pos in line.char_indices() {
			if pos.1.is_digit(10){
				value_collator.0 = value_collator.0 * 10 + pos.1.to_digit(10).unwrap();
				value_collator.1 = pos.0 - value_collator.2;
				value_collator.2 += 1;
			}
			else if value_collator.2 != 0 {
				values.push(Value{num: value_collator.0, len: value_collator.2, pos: Point{x: value_collator.1, y: i}});
				value_collator.0 = 0;
				value_collator.1 = 0;
				value_collator.2 = 0;
			}
		}
		if value_collator.2 != 0 {
			values.push(Value{num: value_collator.0, len: value_collator.2, pos: Point{x: value_collator.1, y: i}});
			value_collator.0 = 0;
			value_collator.1 = 0;
			value_collator.2 = 0;
		}
		i += 1;
	}
	for val in values {
		if scan_neighbours(&val, &grid){
			collator += val.num;
		}
	}

	return collator.to_string()
}