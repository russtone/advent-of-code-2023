use core::num;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut data = Vec::new();

    for line in lines {
        let line = line?;
        let (left, right) = line.split_once(" @ ").ok_or(Error::Parse)?;
        let point = left.parse::<Point>()?;
        let veclocity = right.parse::<Veclocity>()?;
        data.push(Hailstone::new(point, veclocity));
    }

    let mut res = 0;
    let min = 200000000000000.0;
    let max = 400000000000000.0;

    for i in 0..data.len() {
        for j in i..data.len() {
            let a = data[i];
            let b = data[j];
            if let Some(p) = a.line().intersection(&b.line()) {
                if a.is_future(p) && b.is_future(p) {
                    if p.0 >= min && p.0 <= max && p.1 >= min && p.1 <= max {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{:?}", res);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Line {
    a: f64,
    b: f64,
}

impl Line {
    fn intersection(&self, other: &Line) -> Option<(f64, f64)> {
        if self.a == other.a {
            return None;
        }
        let x = (self.b - other.b) / (other.a - self.a);
        let y = self.a * x + self.b;
        return Some((x, y));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Hailstone {
    point: Point,
    velocity: Veclocity,
}

impl Hailstone {
    fn new(point: Point, velocity: Veclocity) -> Self {
        Self { point, velocity }
    }

    fn line(&self) -> Line {
        assert!(self.velocity.vx != 0 && self.velocity.vy != 0);
        let a: f64 = self.velocity.vy as f64 / self.velocity.vx as f64;
        let b = self.point.y as f64 - self.point.x as f64 * a;
        return Line { a, b };
    }

    fn is_future(&self, p: (f64, f64)) -> bool {
        return ((self.velocity.vx > 0 && p.0 > self.point.x as f64)
            || (self.velocity.vx < 0 && p.0 < self.point.x as f64))
            && ((self.velocity.vy > 0 && p.1 > self.point.y as f64)
                || (self.velocity.vy < 0 && p.1 < self.point.y as f64));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(", ").into_iter();
        Ok(Self {
            x: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<isize>()?,
            y: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<isize>()?,
            z: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<isize>()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Veclocity {
    vx: isize,
    vy: isize,
    vz: isize,
}

impl FromStr for Veclocity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(", ").into_iter();
        Ok(Self {
            vx: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<isize>()?,
            vy: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<isize>()?,
            vz: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<isize>()?,
        })
    }
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    Parse,
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
