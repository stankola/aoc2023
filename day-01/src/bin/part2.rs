use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut collator = 0;
    let mut catenator: String = "".to_string();
	let dict: HashMap<&str, u32> =
		[("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
			("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
		].iter().cloned().collect();

    for line in input.lines(){
		'front: for i in 0..line.len(){
			for (&num, &j) in &dict{
				if line[i..].starts_with(num){
					catenator.push(char::from_digit(j, 10).unwrap());
					break 'front;
				} else if char::is_numeric(line.as_bytes()[i] as char){
					catenator.push(line.as_bytes()[i] as char);
					break 'front;
				}
			}
		}
		'back: for i in (0..line.len()).rev(){
			for (&num, &j) in &dict{
				if line[..i + 1].ends_with(num){
					catenator.push(char::from_digit(j, 10).unwrap());
					break 'back;
				} else if char::is_numeric(line.as_bytes()[i] as char){
					catenator.push(line.as_bytes()[i] as char);
					break 'back;
				}
			}
		}
		if !catenator.is_empty(){
			collator += catenator.parse::<i32>().unwrap();
			catenator.clear();
		}
    }
    collator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(include_str!("./input.txt"));
        assert_eq!(result, "55358");
    }
}