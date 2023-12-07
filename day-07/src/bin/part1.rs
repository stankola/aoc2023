use std::collections::HashMap;
use std::cmp::Ordering::*;
use crate::Card::*;

fn main() {
    let test_input = include_str!("./input.txt");
    let output = part1(test_input);
    dbg!(output);
}

 #[derive(Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Debug)]
 enum Type {
    High = 0,
    Pair,
    Pairs,
    Threes,
    FullHouse,
    Fours,
    Fives
 }
 
 #[derive(Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Debug)]
 enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
 }

#[derive(Debug)]
struct Hand {
    cards:[Card; 5]
}

impl Hand {
    fn get_type(&self) -> Type {
        let mut card_counts:Vec<(u32, Card)> = vec![];

        'hand_loop: for hand_card in self.cards {
            for (i, count) in card_counts.iter().enumerate() {
                if count.1 == hand_card {
                    card_counts[i].0 += 1;
                    continue 'hand_loop;
                }
            }
            card_counts.push((1, hand_card));
        }
        card_counts.sort_by(|(a, _), (b, _)| b.cmp(a));
        match card_counts[0].0 {
            5 => Type::Fives,
            4 => Type::Fours,
            3 => if card_counts[1].0 == 2 {Type::FullHouse} else {Type::Threes},
            2 => if card_counts[1].0 == 2 {Type::Pairs} else {Type::Pair},
            1 => Type::High,
            _ => panic!()
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Greater => return Greater,
            Less => return Less,
            _ => ()
        }
        for (i, card) in self.cards.iter().enumerate() {
            match card.cmp(&other.cards[i]) {
                Greater => return Greater,
                Less => return Less,
                _ => ()
            }
        }
        return Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand { }

#[derive(Debug)]
struct Game {
    hand:Hand,
    bid:u32
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Game { }



fn parse_line(line: &str) -> Game {
    let char_to_value:HashMap<char, Card> = HashMap::from({[
        ('2', Two),
        ('3', Three),
        ('4', Four),
        ('5', Five),
        ('6', Six),
        ('7', Seven),
        ('8', Eight),
        ('9', Nine),
        ('T', Ten),
        ('J', Jack),
        ('Q', Queen),
        ('K', King),
        ('A', Ace)
    ]});
    
    let mut hand:Hand = Hand{cards: [Two; 5]};
    let args:Vec<&str> = line.split_whitespace().collect();
    for (i, card) in args[0].chars().enumerate() {
        hand.cards[i] = char_to_value.get(&card).unwrap().clone();
    }
    Game{hand: hand, bid: args[1].parse::<u32>().unwrap()}
}

fn part1(input: &str) -> String {
    let mut collator = 0;

    let mut games:Vec<Game> = vec![];

    for line in input.lines() {
        games.push(parse_line(line));
    }
    games.sort();
    for (i, game) in games.iter().enumerate() {
        collator += (i + 1) as u32 * game.bid;
    }

    collator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let result = part1(include_str!("./test_input.txt"));
        assert_eq!(result, "6440");
    }
}