use core::num;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom},
};

fn main() -> Result<(), Error> {
    let mut file = File::open("files/input.txt")?;

    println!("Part 1: {}", solve(&mut file, parse1)?);

    file.seek(SeekFrom::Start(0))?;

    println!("Part 2: {}", solve(&mut file, parse2)?);

    Ok(())
}

fn solve<R: Read>(
    buf: &mut R,
    parse: fn(&str) -> Result<Instruction, Error>,
) -> Result<u32, Error> {
    let reader = BufReader::new(buf);

    let instructions: Result<Vec<Instruction>, Error> =
        reader.lines().map(|line| parse(&line?)).collect();

    let instructions = instructions?;

    calc(&instructions)
}

fn calc(instructions: &[Instruction]) -> Result<u32, Error> {
    let mut cur = (0, 0);
    let mut border: HashSet<(isize, isize)> = HashSet::new();
    border.insert(cur);

    for inst in instructions {
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
        println!("{}/{}", row, max_row);
    }

    Ok(res)
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

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

fn parse1(s: &str) -> Result<Instruction, Error> {
    let mut parts = s.split_whitespace();

    let direction_str = parts.next().ok_or(Error::ParseError(
        format!("direction is missing: {}", s).to_owned(),
    ))?;

    let direction: Direction = match direction_str {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => {
            return Err(Error::ParseError(
                format!("invalid direction {}", s).to_owned(),
            ))
        }
    };

    let steps: usize = parts
        .next()
        .ok_or(Error::ParseError(
            format!("steps missing: {}", s).to_owned(),
        ))?
        .parse()?;

    Ok(Instruction { direction, steps })
}

fn parse2(s: &str) -> Result<Instruction, Error> {
    let mut parts = s.split_whitespace();
    parts.next();
    parts.next();

    let color = parts.next().ok_or(Error::ParseError(
        format!("color is missing: {}", s).to_owned(),
    ))?;

    let hex = &color[2..color.len() - 1];
    let steps = usize::from_str_radix(&hex[0..5], 16)?;
    let direction = match hex.chars().last().unwrap() {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => {
            return Err(Error::ParseError(
                format!("invalid direction {}", s).to_owned(),
            ))
        }
    };

    Ok(Instruction { direction, steps })
}
