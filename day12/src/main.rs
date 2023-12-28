use core::num;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let mut lines: Vec<Line> = Vec::new();

    for s in BufReader::new(file).lines() {
        lines.push(s?.parse()?)
    }

    let mut res = 0;

    for line in &lines {
        let max = 2_u32.pow(line.unknown.len() as u32);
        let mut val;

        for i in 0..max {
            val = line.data;
            for (j, pos) in line.unknown.iter().enumerate() {
                if i & (1 << j) != 0 {
                    val |= 1 << pos;
                }
            }
            if checksum(val) == line.checksum {
                res += 1;
            }
        }
    }

    println!("{}", res);

    Ok(())
}

fn checksum(n: u32) -> Vec<u32> {
    let mut res = Vec::new();
    let mut m: u32 = n;
    let mut x = 0;

    while m != 0 {
        if m & 0x10000000 != 0 {
            x += 1
        } else {
            if x != 0 {
                res.push(x);
                x = 0
            }
        }
        m <<= 1;
    }
    if x != 0 {
        res.push(x)
    }

    res
}

trait Pad {
    fn pad_left(&self, width: usize, char: char) -> String;
}

impl Pad for &str {
    fn pad_left(&self, width: usize, char: char) -> String {
        let padding_needed = width.saturating_sub(self.len());
        let padding = char.to_string().repeat(padding_needed);

        format!("{}{}", padding, self)
    }
}

#[derive(Debug)]
struct Line {
    data: u32,
    checksum: Vec<u32>,
    unknown: Vec<u32>,
}

impl FromStr for Line {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" ").ok_or(Error::ParseLine)?;

        let mut checksum = Vec::new();
        for ns in right.split(",") {
            checksum.push(ns.parse()?)
        }

        let s = left.pad_left(32, '.');

        let data = s
            .chars()
            .map(|ch| match ch {
                '.' | '?' => 0,
                _ => 1,
            })
            .fold(0, |acc, bit| (acc << 1) | bit);

        let unknown = s
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, ch)| *ch == '?')
            .map(|(i, _)| i as u32)
            .collect();

        Ok(Line {
            data,
            checksum,
            unknown,
        })
    }
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseLine,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(7), vec![3]);
        assert_eq!(checksum(16190), vec![6, 5]);
    }
}
