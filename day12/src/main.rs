use core::num;
use std::io::{self, BufRead};

fn main() -> Result<(), Error> {
    println!("Part 1: {}", part1::solve()?);
    println!("Part 2: {}", part2::solve()?);

    Ok(())
}
mod part2 {

    use super::*;
    use core::hash::Hash;
    use std::{
        cmp,
        collections::{hash_map::DefaultHasher, HashMap},
        fs::File,
        hash::Hasher,
        io::BufReader,
        str::FromStr,
    };

    pub fn solve() -> Result<u64, Error> {
        let file = File::open("files/input.txt")?;
        let mut lines: Vec<Line> = Vec::new();

        for s in BufReader::new(file).lines() {
            lines.push(s?.parse()?)
        }

        let mut res = 0;

        for (i, line) in lines.iter().enumerate() {
            let mut c = Counter::new();
            res += c.count(&line.data, &line.checksum);
        }
        Ok(res)
    }

    fn hash_args<T: Hash + ?Sized, U: Hash + ?Sized>(a: &T, b: &U) -> u64 {
        let mut hasher = DefaultHasher::new();
        a.hash(&mut hasher);
        b.hash(&mut hasher);
        hasher.finish()
    }

    struct Counter {
        cache: HashMap<u64, u64>,
    }

    impl Counter {
        fn new() -> Self {
            Counter {
                cache: HashMap::new(),
            }
        }

        fn count(&mut self, data: &[char], checksum: &[usize]) -> u64 {
            let key = hash_args(data, checksum);

            match self.cache.get(&key) {
                Some(res) => *res,
                None => {
                    let res = match (data, checksum) {
                        ([..], []) => {
                            if data.contains(&'#') {
                                0
                            } else {
                                1
                            }
                        }
                        (_, [head, tail @ ..]) => {
                            let max_offset =
                                data.len() - (tail.iter().sum::<usize>() + head + tail.len() - 1);
                            let mut res = 0;
                            for start in 0..max_offset {
                                let end = start + head;

                                if data[..start].contains(&'#') {
                                    break;
                                }

                                if end <= data.len()
                                    && !data[start..end].contains(&'.')
                                    && data.get(end) != Some(&'#')
                                {
                                    res += self.count(&data[cmp::min(end + 1, data.len())..], &tail)
                                }
                            }
                            res
                        }
                    };
                    self.cache.insert(key, res);
                    res
                }
            }
        }
    }

    #[derive(Debug)]
    struct Line {
        data: Vec<char>,
        checksum: Vec<usize>,
    }

    impl FromStr for Line {
        type Err = Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (left, right) = s.split_once(" ").ok_or(Error::ParseLine)?;

            let mut checksum = Vec::new();
            for ns in right.split(",") {
                checksum.push(ns.parse()?)
            }
            checksum = checksum.repeat(5);

            let mut data: Vec<char> = left.chars().collect();
            data.push('?');
            data = data.repeat(5);
            data.pop();

            Ok(Line { data, checksum })
        }
    }
}

mod part1 {
    use super::*;
    use std::{fs::File, io::BufReader, str::FromStr};

    pub fn solve() -> Result<u32, Error> {
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
        Ok(res)
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_checksum() {
            assert_eq!(checksum(7), vec![3]);
            assert_eq!(checksum(16190), vec![6, 5]);
        }
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
