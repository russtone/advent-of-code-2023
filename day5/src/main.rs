use core::num;
use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    println!("Part 1: {}", part1()?);
    println!("Part 2: {}", part2()?);
    Ok(())
}

fn part1() -> Result<u64, Error> {
    let file = File::open("./files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    let mut map: Option<Map> = None;
    let mut min: u64 = u64::MAX;

    for line in lines {
        let line = line?;
        if line.starts_with("seeds: ") {
            seeds = parse_seeds(line.strip_prefix("seeds: ").unwrap())?;
        } else if line.ends_with(" map:") {
            map = parse_map(line.strip_suffix(" map:").unwrap()).ok();
        } else if line.is_empty() {
            if let Some(map) = map {
                maps.push(map);
            }
            map = None
        } else {
            if let Some(ref mut map) = map {
                map.ranges.push(parse_range(&line)?);
            }
        }
    }
    if let Some(map) = map {
        maps.push(map);
    }

    for seed in seeds.iter() {
        let mut res: u64 = *seed;
        for map in maps.iter() {
            res = map.apply(res)
        }
        if res < min {
            min = res
        }
    }

    Ok(min)
}

fn part2() -> Result<u64, Error> {
    let file = File::open("./files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    let mut map: Option<Map> = None;
    let mut min: u64 = u64::MAX;

    for line in lines {
        let line = line?;
        if line.starts_with("seeds: ") {
            seed_ranges = parse_seed_ranges(line.strip_prefix("seeds: ").unwrap())?;
        } else if line.ends_with(" map:") {
            map = parse_map(line.strip_suffix(" map:").unwrap()).ok();
        } else if line.is_empty() {
            if let Some(map) = map {
                maps.push(map);
            }
            map = None
        } else {
            if let Some(ref mut map) = map {
                map.ranges.push(parse_range(&line)?);
            }
        }
    }
    if let Some(map) = map {
        maps.push(map);
    }

    for (start, count) in seed_ranges {
        for seed in start..(start + count) {
            let mut res: u64 = seed;
            for map in maps.iter() {
                res = map.apply(res)
            }
            if res < min {
                min = res
            }
        }
    }

    Ok(min)
}

fn parse_seeds(s: &str) -> Result<Vec<u64>, Error> {
    let mut nums: Vec<u64> = Vec::new();
    for ns in s.split_whitespace().into_iter() {
        nums.push(ns.parse()?);
    }
    Ok(nums)
}

fn parse_seed_ranges(s: &str) -> Result<Vec<(u64, u64)>, Error> {
    let mut res: Vec<(u64, u64)> = Vec::new();

    for (start, count) in s.split_whitespace().into_iter().tuples().into_iter() {
        res.push((start.parse()?, count.parse()?));
    }
    Ok(res)
}

fn parse_map(s: &str) -> Result<Map, Error> {
    let (from, to) = s
        .split_once("-to-")
        .ok_or(Error::Parse("fail to parse map name".to_owned()))?;
    Ok(Map::new(from, to))
}

fn parse_range(s: &str) -> Result<Range, Error> {
    let mut parts = s.split(" ");
    let dst = parts.next().unwrap().parse()?;
    let src = parts.next().unwrap().parse()?;
    let count = parts.next().unwrap().parse()?;

    Ok(Range {
        source: src,
        destination: dst,
        count,
    })
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    Parse(String),
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Error {
        Error::Io(other)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(other: num::ParseIntError) -> Error {
        Error::ParseInt(other)
    }
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Map {
    fn new(from: &str, to: &str) -> Map {
        Map {
            from: from.to_owned(),
            to: to.to_owned(),
            ranges: Vec::new(),
        }
    }

    fn apply(&self, src: u64) -> u64 {
        for r in self.ranges.iter() {
            if r.contains(src) {
                return r.apply(src);
            }
        }
        return src;
    }
}

#[derive(Debug)]
struct Range {
    source: u64,
    destination: u64,
    count: u64,
}

impl Range {
    fn contains(&self, src: u64) -> bool {
        src >= self.source && src < self.source + self.count
    }

    fn apply(&self, src: u64) -> u64 {
        self.destination + (src - self.source)
    }
}
