use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut sum: u32 = 0;

    for line in lines {
        let card = Card::from_string(&line?).unwrap();
        sum += card.points();
    }

    println!("Answer: {}", sum);

    Ok(())
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

    fn points(&self) -> u32 {
        let mut li: usize = 0;
        let mut ri: usize = 0;
        let mut points: u32 = 0;

        while li < self.left.len() && ri < self.right.len() {
            let lv = self.left[li];
            let rv = self.right[ri];

            if lv == rv {
                points = if points == 0 { 1 } else { points * 2 };
                li += 1;
                ri += 1;
            } else if lv < rv {
                li += 1
            } else if lv > rv {
                ri += 1
            }
        }

        return points;
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
