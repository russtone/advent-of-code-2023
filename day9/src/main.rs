use core::num;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut res: i32 = 0;

    for line in lines {
        let line = line?;
        let mut nums = parse_line(&line)?;
        let mut vecs: Vec<Vec<i32>> = vec![nums.clone()];

        while !nums.iter().all(|x| *x == 0) {
            nums = nums
                .windows(2)
                .into_iter()
                .map(|x| x[1] - x[0])
                .collect::<Vec<i32>>();
            vecs.push(nums.clone());
        }

        let mut n: i32 = 0;
        for v in vecs.iter().rev().skip(1) {
            n += v.last().unwrap();
        }
        res += n;
    }

    println!("{}", res);

    Ok(())
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Self {
        Error::Parse(value)
    }
}

fn parse_line(s: &str) -> Result<Vec<i32>, num::ParseIntError> {
    let mut nums: Vec<i32> = Vec::new();

    for ns in s.split_whitespace() {
        nums.push(ns.parse()?);
    }

    return Ok(nums);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_nums() {
        let s: &str = "1 2 3 4";
        assert_eq!(parse_line(s), Ok(vec![1, 2, 3, 4]));

        let s: &str = "1 -2 3 -4";
        assert_eq!(parse_line(s), Ok(vec![1, -2, 3, -4]));
    }
}
