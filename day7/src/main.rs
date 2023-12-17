use core::num;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut rows: Vec<Row> = Vec::new();

    for line in lines {
        let line = line?;
        rows.push(line.parse()?);
    }

    rows.sort_by_key(|r| r.hand.get_combination());

    let mut res: u32 = 0;
    for (i, row) in rows.iter().enumerate() {
        let rank: u32 = (i + 1) as u32;
        res += row.bid * rank;
        println!("{} {} {:?}", rank, row.bid, row.hand.get_combination());
    }

    println!("Answer: {}", res);

    Ok(())
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseRow(ParseRowError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}

impl From<ParseRowError> for Error {
    fn from(value: ParseRowError) -> Self {
        Error::ParseRow(value)
    }
}

#[derive(Debug)]
struct Row {
    hand: Hand,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseRowError {
    WrongRowFormat,
    Hand(ParseHandError),
    ParseInt(num::ParseIntError),
}

impl From<ParseHandError> for ParseRowError {
    fn from(value: ParseHandError) -> Self {
        ParseRowError::Hand(value)
    }
}

impl From<num::ParseIntError> for ParseRowError {
    fn from(value: num::ParseIntError) -> Self {
        ParseRowError::ParseInt(value)
    }
}

impl FromStr for Row {
    type Err = ParseRowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_once(" ").ok_or(ParseRowError::WrongRowFormat)?;

        let hand: Hand = hand_str.parse()?;
        let bid: u32 = bid_str.parse()?;

        Ok(Row { hand, bid })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Combination {
    Nothing(Card, Card, Card, Card, Card),
    Pair(Card, (Card, Card, Card)),
    TwoPairs(Card, Card, Card),
    Three(Card, (Card, Card)),
    FullHouse(Card, Card),
    Four(Card, Card),
    Five(Card),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn get_combination(&self) -> Combination {
        let mut prev: Card = self.cards[0];
        let mut count: u32 = 1;
        let mut cards: Vec<(Card, u32)> = Vec::new();

        for i in 1..5 {
            let card = self.cards[i];
            if prev == card {
                count += 1;
            } else {
                cards.push((prev, count));
                count = 1;
            }
            prev = card;
        }
        cards.push((prev, count));

        cards.sort_by_key(|c| c.1);

        match cards[..] {
            [(c1, 1), (c2, 1), (c3, 1), (c4, 1), (c5, 1)] => {
                Combination::Nothing(c5, c4, c3, c2, c1)
            }
            [(c1, 1), (c2, 1), (c3, 1), (c4, 2)] => Combination::Pair(c4, (c3, c2, c1)),
            [(c1, 1), (c2, 2), (c3, 2)] => Combination::TwoPairs(c3, c2, c1),
            [(c1, 1), (c2, 1), (c3, 3)] => Combination::Three(c3, (c2, c1)),
            [(c1, 2), (c2, 3)] => Combination::FullHouse(c2, c1),
            [(c1, 1), (c2, 4)] => Combination::Four(c2, c1),
            [(c1, 5)] => Combination::Five(c1),
            _ => panic!("invalid combination"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseHandError {
    WrongNumberOfCards,
    Card(ParseCardError),
}

impl From<ParseCardError> for ParseHandError {
    fn from(value: ParseCardError) -> Self {
        ParseHandError::Card(value)
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(ParseHandError::WrongNumberOfCards);
        }

        let mut cards: [Card; 5] = [Card::Two; 5];

        for (i, c) in s.chars().enumerate() {
            let card: Card = c.to_string().parse()?;
            cards[i] = card;
        }

        cards.sort();

        return Ok(Hand { cards });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
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
    Ace,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError {
    wrong: String,
}

impl FromStr for Card {
    type Err = ParseCardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            v => Err(ParseCardError {
                wrong: v.to_owned(),
            }),
        }
    }
}
