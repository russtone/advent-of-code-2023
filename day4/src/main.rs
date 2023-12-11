use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1()?);
    println!("Part 2: {}", part2()?);

    Ok(())
}

fn part1() -> io::Result<u32> {
    let file = File::open("./files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut points: u32 = 0;

    let base: u32 = 2;
    for line in lines {
        let card = Card::from_string(&line?).unwrap();
        let matches = card.matches();
        points += if matches == 0 {
            0
        } else {
            base.pow(matches - 1)
        };
    }

    Ok(points)
}

fn part2() -> io::Result<u32> {
    let file = File::open("./files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut cards_count: u32 = 0;
    let mut counters: Vec<u32> = Vec::new();

    for line in lines {
        let card = Card::from_string(&line?).unwrap();

        let mut copies_count: u32 = 1;
        for c in counters.iter_mut() {
            copies_count += 1;
            *c -= 1;
        }
        counters.retain(|c| *c != 0);

        cards_count += copies_count;

        let matches = card.matches();

        if matches > 0 {
            for _ in 0..copies_count {
                counters.push(matches);
            }
        }
    }

    Ok(cards_count)
}

#[derive(Debug, Clone)]
struct Card {
    index: u32,
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Card {
    fn from_string(s: &str) -> Option<Card> {
        let (card_info_str, numbers_str) = s.split_once(": ")?;
        let card_index_str = card_info_str.split_whitespace().into_iter().last()?;
        let (left_str, right_str) = numbers_str.split_once(" | ")?;

        Some(Card {
            index: card_index_str.parse().ok()?,
            left: parse_numbers(&left_str)?,
            right: parse_numbers(&right_str)?,
        })
    }

    fn matches(&self) -> u32 {
        let mut li: usize = 0;
        let mut ri: usize = 0;
        let mut matches: u32 = 0;

        while li < self.left.len() && ri < self.right.len() {
            let lv = self.left[li];
            let rv = self.right[ri];

            if lv == rv {
                matches += 1;
                li += 1;
                ri += 1;
            } else if lv < rv {
                li += 1
            } else if lv > rv {
                ri += 1
            }
        }

        return matches;
    }
}

fn parse_numbers(s: &str) -> Option<Vec<u32>> {
    let mut nums = Vec::new();

    for num_str in s.split_whitespace().into_iter() {
        let num = num_str.parse().ok()?;
        nums.push(num)
    }
    nums.sort();

    return Some(nums);
}
