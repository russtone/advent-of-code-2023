use core::num;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let reader = BufReader::new(file);

    let instructions: Result<Vec<Instruction>, Error> = reader
        .lines()
        .map(|line| line?.parse::<Instruction>())
        .collect();

    let instructions = instructions?;

    let mut cur = (0, 0);
    let mut border: HashSet<(isize, isize)> = HashSet::new();
    border.insert(cur);

    for inst in &instructions {
        for _ in 0..inst.steps {
            match inst.direction {
                Direction::Up => cur.0 -= 1,
                Direction::Down => cur.0 += 1,
                Direction::Left => cur.1 -= 1,
                Direction::Right => cur.1 += 1,
            }
            border.insert(cur);
        }
    }

    let min_row = border.iter().map(|p| p.0).min().unwrap();
    let min_col = border.iter().map(|p| p.1).min().unwrap();

    let max_row = border.iter().map(|p| p.0).max().unwrap() + 1;
    let max_col = border.iter().map(|p| p.1).max().unwrap() + 1;

    let mut res = 0;


    for row in min_row..max_row {
        let mut cross = 0;
        let mut border_start: Option<isize> = None;

        for col in min_col..max_col {
            if border.contains(&(row, col)) {
                if border_start.is_none() {
                    border_start = Some(col);
                }
                res += 1;
                continue;
            }

            if let Some(start) = border_start {
                if row != min_row && row != max_row - 1 {
                    let end = col - 1;
                    if (border.contains(&(row - 1, start)) && border.contains(&(row + 1, end)))
                        || (border.contains(&(row + 1, start)) && border.contains(&(row - 1, end)))
                    {
                        cross += 1;
                    }
                }
                border_start = None;
            }

            if cross % 2 == 1 {
                res += 1;
            }
        }
    }

    println!("{}", res);

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("io error")]
    IO(#[from] io::Error),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("parse int error")]
    ParseInt(#[from] num::ParseIntError),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(Error::ParseError(
                format!("invalid direction {}", s).to_owned(),
            )),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    steps: usize,
    color: String,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let direction: Direction = parts
            .next()
            .ok_or(Error::ParseError(
                format!("direction is missing: {}", s).to_owned(),
            ))?
            .parse()?;

        let steps: usize = parts
            .next()
            .ok_or(Error::ParseError(
                format!("steps missing: {}", s).to_owned(),
            ))?
            .parse()?;

        let color = parts.next().ok_or(Error::ParseError(
            format!("color is missing: {}", s).to_owned(),
        ))?;

        Ok(Instruction {
            direction,
            steps,
            color: color[1..color.len() - 1].to_owned(),
        })
    }
}
