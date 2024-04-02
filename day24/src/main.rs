use core::num;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

use nalgebra::{ComplexField, Matrix4, Vector4};

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

    println!("Part 1: {:?}", part1(&data)?);
    println!("Part 2: {:?}", part2(&data)?);

    Ok(())
}

/*
  1. (X - x) / (dx - DX) = (Y - y) / (dy - DY)
  2. (X - x) * (dy - DY) = (Y - y) * (dx - DX)
  3. X * dy - X * DY - x * dy + x * DY = Y * dx - Y * DX - y * dx + y * DX
  4. Y * DX - X * DY = Y * dx - X * dy + y * DX - x * DY + x * dy - y * dx
  5. Y * DX - X * DY = Y * dx' - X * dy' + y' * DX - x' * DY + x' * dy - y' * dx
  6. X * (dy' - dy) + Y * (dx - dx') + DX * (y - y') + DY * (x' - x) + x * dy - x' * dy' - y * dx + y' * dx' = 0
*/
fn part2(data: &[Hailstone]) -> Result<u64, Error> {
    let h = data.first().unwrap();

    let mut coeffs1 = Vec::new();
    let mut consts1 = Vec::new();
    let mut coeffs2 = Vec::new();
    let mut consts2 = Vec::new();

    for i in 2..=5 {
        let h1 = data[i];
        coeffs1.push(vec![
            h1.velocity.vy - h.velocity.vy,
            h.velocity.vx - h1.velocity.vx,
            h.point.y - h1.point.y,
            h1.point.x - h.point.x,
        ]);
        consts1.push(
            h.point.x * h.velocity.vy - h1.point.x * h1.velocity.vy - h.point.y * h.velocity.vx
                + h1.point.y * h1.velocity.vx,
        );

        coeffs2.push(vec![
            h1.velocity.vz - h.velocity.vz,
            h.velocity.vx - h1.velocity.vx,
            h.point.z - h1.point.z,
            h1.point.x - h.point.x,
        ]);
        consts2.push(
            h.point.x * h.velocity.vz - h1.point.x * h1.velocity.vz - h.point.z * h.velocity.vx
                + h1.point.z * h1.velocity.vx,
        );
    }

    let s1 = solve(&coeffs1.concat(), &consts1);
    let s2 = solve(&coeffs2.concat(), &consts2);

    Ok((-s1[0] - s1[1] - s2[1]) as u64)
}

fn solve(coeffs: &[f64], consts: &[f64]) -> Vec<f64> {
    let matrix = Matrix4::from_row_slice(&coeffs);
    let rhs = Vector4::from_column_slice(&consts);

    let solution = (matrix.try_inverse().unwrap() * rhs)
        .iter()
        .map(|&v| v.round())
        .collect::<Vec<f64>>();

    solution
}

fn part1(data: &[Hailstone]) -> Result<u64, Error> {
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

    Ok(res)
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Hailstone {
    point: Point,
    velocity: Veclocity,
}

impl Hailstone {
    fn new(point: Point, velocity: Veclocity) -> Self {
        Self { point, velocity }
    }

    fn line(&self) -> Line {
        assert!(self.velocity.vx != 0.0 && self.velocity.vy != 0.0);
        let a: f64 = self.velocity.vy as f64 / self.velocity.vx as f64;
        let b = self.point.y as f64 - self.point.x as f64 * a;
        return Line { a, b };
    }

    fn is_future(&self, p: (f64, f64)) -> bool {
        return ((self.velocity.vx > 0.0 && p.0 > self.point.x as f64)
            || (self.velocity.vx < 0.0 && p.0 < self.point.x as f64))
            && ((self.velocity.vy > 0.0 && p.1 > self.point.y as f64)
                || (self.velocity.vy < 0.0 && p.1 < self.point.y as f64));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
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
                .parse::<f64>()?,
            y: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<f64>()?,
            z: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<f64>()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Veclocity {
    vx: f64,
    vy: f64,
    vz: f64,
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
                .parse::<f64>()?,
            vy: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<f64>()?,
            vz: iter
                .next()
                .ok_or(Error::Parse)?
                .trim_start()
                .parse::<f64>()?,
        })
    }
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    ParseFloat(num::ParseFloatError),
    Parse,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(value: num::ParseFloatError) -> Self {
        Error::ParseFloat(value)
    }
}
