fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut collator = 0;
    let mut catenator: String = "".to_string();

    for line in input.lines(){
        match line.find(|c: char| c.is_ascii_digit()){
            Some(i) => catenator.push(line.chars().nth(i).unwrap()),
            None => (),
        }
        match line.rfind(|c: char| c.is_ascii_digit()){
            Some(i) => catenator.push(line.chars().nth(i).unwrap()),
            None => (),
        }
        collator += catenator.parse::<i32>().unwrap();
        catenator.clear();
    }
    collator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("./input.txt"));
        assert_eq!(result, "56042");
    }
}