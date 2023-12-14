use core::num;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    println!("Part 1: {}", solve(parse_numbers)?);
    println!("Part 2: {}", solve(parse_numbers2)?);
    Ok(())
}

fn solve(parse_nums: fn(&str) -> Result<Vec<u64>, Error>) -> Result<u64, Error> {
    let file = File::open("files/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    for line in lines {
        let line = line?;
        if line.starts_with("Time: ") {
            times = parse_nums(line.strip_prefix("Time: ").unwrap())?;
        } else if line.starts_with("Distance: ") {
            distances = parse_nums(line.strip_prefix("Distance: ").unwrap())?;
        }
    }

    let mut res: u64 = 1;

    for (t, d) in times.iter().zip(distances).into_iter() {
        let eq = SquareEq::new(1.0, -(*t as f64), d as f64);
        if let Some(roots) = eq.roots() {
            let mut low: u64 = roots.0.ceil() as u64;
            let mut high: u64 = roots.1.floor() as u64;
            if low as f64 == roots.0 {
                low += 1;
            }
            if high as f64 == roots.1 {
                high -= 1;
            }
            res *= high - low + 1;
        }
    }

    Ok(res)
}

fn parse_numbers(s: &str) -> Result<Vec<u64>, Error> {
    let mut nums: Vec<u64> = Vec::new();

    for ns in s.split_whitespace().into_iter() {
        nums.push(ns.parse()?)
    }

    return Ok(nums);
}

fn parse_numbers2(s: &str) -> Result<Vec<u64>, Error> {
    let mut nums: Vec<u64> = Vec::new();
    nums.push(s.replace(" ", "").parse()?);
    return Ok(nums);
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    ParseInt(num::ParseIntError),
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

#[derive(Debug)]
struct SquareEq {
    a: f64,
    b: f64,
    c: f64,
}

impl SquareEq {
    fn new(a: f64, b: f64, c: f64) -> SquareEq {
        SquareEq { a, b, c }
    }

    fn discriminant(&self) -> f64 {
        self.b.powi(2) - 4.0 * self.a * self.c
    }

    fn roots(&self) -> Option<(f64, f64)> {
        let d = self.discriminant();
        if d < 0.0 {
            return None;
        }

        let r1 = (-self.b - d.sqrt()) / 2.0 * self.a;
        let r2 = (-self.b + d.sqrt()) / 2.0 * self.a;

        Some((r1, r2))
    }
}
