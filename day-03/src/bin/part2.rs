use std::ops::Range;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

//#[derive(Copy, Clone)]
struct Value {
	num: u32,
	pos: Point,
	len: usize
}

struct Gear {
	pos: Point
}

//#[derive(Copy, Clone)]
struct Point {
	x: usize,
	y: usize
}

fn scan_neighbours(val: &Value, grid: &Vec::<Vec<char>>) -> bool {
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

fn part2(input: &str) -> String {
	let mut collator = 0;

	let mut grid: Vec::<Vec<char>> = vec![];
	let mut values: Vec::<Value> = vec![];
	let mut gears: Vec::<Gear> = vec![];

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
			if pos.1 == '*' {
				gears.push(Gear{pos: Point{x:pos.0, y:i}});
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
	for gear in gears {
		println!("gear y:{} x:{}", gear.pos.y, gear.pos.x);
		let mut matches: Vec<&Value> = vec![];
		let xrange = Range{start: gear.pos.x - 1, end: gear.pos.x + 2};
		let yrange = Range{start: gear.pos.y - 1, end: gear.pos.y + 2};
		for value in values.iter() {
			println!("matching value {} y:{} x:{}", value.num, value.pos.y, value.pos.x);
			if yrange.contains(&value.pos.y) {
				'inner: for j in 0..value.len {
					let p = value.pos.x + j;
					if xrange.contains(&p){
						println!("match found y:{} x:{}", value.pos.y, value.pos.x);
						matches.push(&value);
						break 'inner;
					}
				}
			}
		}
		println!("matches {}", matches.len());
		if matches.len() == 2 {
			collator += matches[0].num * matches[1].num;
		}
	}

	return collator.to_string()
}