use core::num;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
};

fn main() -> Result<(), Error> {
    let mut file = File::open("files/input.txt")?;

    println!("{}", ext(&mut file, next)?);
    file.seek(SeekFrom::Start(0))?;
    println!("{}", ext(&mut file, prev)?);

    Ok(())
}

fn ext<R: Read>(buf: &mut R, f: fn(&Vec<Vec<i32>>) -> i32) -> Result<i32, Error> {
    let lines = BufReader::new(buf).lines();
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

        res += f(&vecs);
    }
    Ok(res)
}

fn next(vecs: &Vec<Vec<i32>>) -> i32 {
    let mut n: i32 = 0;
    for v in vecs.iter().rev().skip(1) {
        n += v.last().unwrap();
    }
    n
}

fn prev(vecs: &Vec<Vec<i32>>) -> i32 {
    let mut n: i32 = 0;
    for v in vecs.iter().rev().skip(1) {
        n = v.first().unwrap() - n;
    }
    n
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

    #[test]
    fn test_next() {
        assert_eq!(ext(&mut Cursor::new("1 2 3 4"), next).unwrap(), 5);
    }

    #[test]
    fn test_prev() {
        assert_eq!(ext(&mut Cursor::new("1 2 3 4"), prev).unwrap(), 0);
        assert_eq!(ext(&mut Cursor::new("1 4 7 10"), prev).unwrap(), -2);
    }
}
