use core::num;
use std::{
    cmp::min,
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let mut lines = BufReader::new(file).lines().peekable();
    let mut maps: Vec<Map> = Vec::new();
    let mut data: Vec<Vec<char>> = Vec::new();

    while let Some(line) = lines.next() {
        let line = line?;
        let last_line = lines.peek().is_none();

        if !line.is_empty() {
            data.push(line.chars().collect());
        }

        if line.is_empty() || last_line {
            maps.push(Map::new(&data));
            data.clear();
        }
    }

    let mut res: usize = 0;

    for map in maps.iter() {
        let mut cols: HashSet<usize> = HashSet::new();

        for (i, row) in map.rows().enumerate() {
            if i == 0 {
                cols = find_possible_reflection_offsets(row);
            } else {
                cols.retain(|offset| check_reflection(row, *offset));
            }
        }

        for col in cols.iter() {
            res += col + 1;
        }

        let mut rows: HashSet<usize> = HashSet::new();

        for (i, col) in map.cols().enumerate() {
            if i == 0 {
                rows = find_possible_reflection_offsets(&col);
            } else {
                rows.retain(|offset| check_reflection(&col, *offset));
            }
        }

        for row in rows.iter() {
            res += (row + 1) * 100;
        }

        if cols.is_empty() && rows.is_empty() {
            panic!("no reflection found:\n{}", map);
        }
    }

    println!("{}", res);

    Ok(())
}

fn find_possible_reflection_offsets(s: &Vec<char>) -> HashSet<usize> {
    let mut offsets = HashSet::new();

    for offset in 0..s.len() - 1 {
        if check_reflection(s, offset) {
            offsets.insert(offset);
        }
    }

    offsets
}

fn check_reflection(s: &Vec<char>, offset: usize) -> bool {
    let depth = min(offset + 1, s.len() - offset - 1);
    for i in 0..depth {
        if s[offset - i] != s[offset + i + 1] {
            return false;
        }
    }
    true
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_reflection() {
        assert_eq!(check_reflection(&"#.##..##.".chars().collect(), 4), true);
        assert_eq!(
            check_reflection(&"..##.###..###.##.".chars().collect(), 0),
            true
        );
    }

    #[test]
    fn test_find_possible_reflection_offsets() {
        assert_eq!(
            find_possible_reflection_offsets(&"#.##..##.".chars().collect()),
            HashSet::from_iter(vec![4, 6])
        );
        assert_eq!(
            find_possible_reflection_offsets(&"..##.###..###.##.".chars().collect()),
            HashSet::from_iter(vec![0, 8, 14])
        );
    }
}
