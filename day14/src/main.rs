use core::num;
use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut data: Vec<Vec<char>> = Vec::new();

    for line in lines {
        data.push(line?.chars().collect());
    }

    let map = Map::new(&data);

    let mut res: usize = 0;

    for col in map.cols() {
        let mut stop: usize = 0;
        for (i, c) in col.iter().enumerate() {
            match c {
                'O' => {
                    res += map.rows - stop;
                    stop = stop + 1;
                }
                '.' => continue,
                '#' => stop = i + 1,
                _ => todo!(),
            }
        }
    }

    println!("{}", res);

    Ok(())
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(data: &Vec<Vec<char>>) -> Self {
        let rows = data.len();
        let cols = data.first().unwrap().len();

        Map {
            data: data.clone(),
            rows,
            cols,
        }
    }

    fn rows(&self) -> RowsIterator {
        RowsIterator { map: &self, row: 0 }
    }

    fn cols(&self) -> ColsIterator {
        ColsIterator { map: &self, col: 0 }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.iter() {
            write!(f, "{}\n", line.iter().collect::<String>())?
        }
        Ok(())
    }
}

struct RowsIterator<'a> {
    map: &'a Map,
    row: usize,
}

impl<'a> Iterator for RowsIterator<'a> {
    type Item = &'a Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.map.data.len() {
            let result = &self.map.data[self.row];
            self.row += 1;
            Some(result)
        } else {
            None
        }
    }
}

struct ColsIterator<'a> {
    map: &'a Map,
    col: usize,
}

impl<'a> Iterator for ColsIterator<'a> {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.map.data.first().unwrap().len() {
            let mut result = Vec::new();
            for row in self.map.rows() {
                result.push(row[self.col])
            }
            self.col += 1;
            Some(result)
        } else {
            None
        }
    }
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
