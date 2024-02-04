use core::num;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom},
};

fn main() -> Result<(), Error> {
    let mut file = File::open("files/input.txt")?;

    println!("Part 1: {}", solve(&mut file, parse1, calc1)?);

    file.seek(SeekFrom::Start(0))?;

    println!("Part 2: {}", solve(&mut file, parse2, calc2)?);

    Ok(())
}

fn solve<R: Read>(
    buf: &mut R,
    parse: fn(&str) -> Result<Instruction, Error>,
    calc: fn(&[Instruction]) -> Result<u64, Error>,
) -> Result<u64, Error> {
    let reader = BufReader::new(buf);

    let instructions: Result<Vec<Instruction>, Error> =
        reader.lines().map(|line| parse(&line?)).collect();

    let instructions = instructions?;

    calc(&instructions)
}

fn calc2(instructions: &[Instruction]) -> Result<u64, Error> {
    let mut cur: (isize, isize) = (0, 0);
    let mut border: BTreeMap<isize, BTreeSet<(isize, isize)>> = BTreeMap::new();
    let mut dir_in: BTreeMap<(isize, isize, isize), Direction> = BTreeMap::new();
    let mut dir_out: BTreeMap<(isize, isize, isize), Direction> = BTreeMap::new();
    let mut prev_dir: Direction = instructions.last().unwrap().direction;
    let mut last_entry: Option<(isize, isize, isize)> = None;
    let mut min_row = 0;
    let mut max_row = 0;
    let mut min_col = 0;
    let mut max_col = 0;

    for inst in instructions {
        if let Some(v) = last_entry {
            dir_out.insert(v, inst.direction);
            last_entry = None;
        }

        match inst.direction {
            Direction::Up => {
                for i in 0..inst.steps {
                    if i != inst.steps - 1 {
                        insert(&mut border, cur.0 - 1, cur.1, cur.1);
                    }
                    cur.0 -= 1;
                }
            }
            Direction::Down => {
                for i in 0..inst.steps {
                    if i != inst.steps - 1 {
                        insert(&mut border, cur.0 + 1, cur.1, cur.1);
                    }
                    cur.0 += 1;
                }
            }
            Direction::Left => {
                let v = (cur.0, cur.1 - inst.steps, cur.1);
                insert(&mut border, cur.0, cur.1 - inst.steps, cur.1);
                dir_in.insert(v, prev_dir);
                last_entry = Some(v);
                cur.1 -= inst.steps;
            }
            Direction::Right => {
                let v = (cur.0, cur.1, cur.1 + inst.steps);
                insert(&mut border, cur.0, cur.1, cur.1 + inst.steps);
                dir_in.insert(v, prev_dir);
                last_entry = Some(v);
                cur.1 += inst.steps;
            }
        }

        if cur.0 < min_row {
            min_row = cur.0;
        }
        if cur.0 > max_row {
            max_row = cur.0;
        }
        if cur.1 < min_col {
            min_col = cur.1;
        }
        if cur.1 > max_col {
            max_col = cur.1;
        }

        prev_dir = inst.direction;
    }

    let mut res: u64 = 0;

    for (row, set) in &border {
        let mut last_end = 0;
        let mut crossings = 0;
        for (start, end) in set {
            let len = (end - start) as u64 + 1;
            res += len;

            if crossings % 2 == 1 {
                res += (start - last_end) as u64 - 1;
            }

            if len > 1 {
                let d1 = dir_in.get(&(*row, *start, *end)).unwrap();
                let d2 = dir_out.get(&(*row, *start, *end)).unwrap();

                if d1 == d2 {
                    crossings += 1;
                }
            } else {
                crossings += 1;
            }
            last_end = *end;
        }
    }

    Ok(res)
}

fn insert(
    border: &mut BTreeMap<isize, BTreeSet<(isize, isize)>>,
    row: isize,
    start: isize,
    end: isize,
) {
    border
        .entry(row)
        .and_modify(|e| {
            e.insert((start, end));
        })
        .or_insert(vec![(start, end)].into_iter().collect());
}

fn print_map(border: &HashSet<(isize, isize)>, rows: (isize, isize), cols: (isize, isize)) {
    for row in rows.0..rows.1 {
        for col in cols.0..cols.1 {
            if border.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn calc1(instructions: &[Instruction]) -> Result<u64, Error> {
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

    // print_map(&border, (min_row, max_row), (min_col, max_col));

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    steps: isize,
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

    let steps: isize = parts
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
    let steps = isize::from_str_radix(&hex[0..5], 16)?;
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
