use core::num;
use std::{
    fmt::Display,
    fs::File,
    hash::{Hash, Hasher},
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut data: Vec<Vec<char>> = Vec::new();

    for line in lines {
        data.push(line?.chars().collect());
    }

    let mut map = Map::new(&data);

    println!("Part 1: {}", part1::solve(&map)?);
    println!("Part 2: {}", part2::solve(&mut map)?);

    Ok(())
}

mod part2 {
    use std::collections::HashMap;

    use super::*;

    pub fn solve(map: &mut Map) -> Result<usize, Error> {
        let mut seen: HashMap<u64, usize> = HashMap::new();
        let mut loop_start = 0;
        let mut loop_len = 0;

        for i in 0..1000000000 {
            let hash = map.hash();

            if let Some(ind) = seen.get(&hash) {
                loop_start = *ind;
                loop_len = i - ind;
                break;
            } else {
                seen.insert(hash, i);
            }

            map.tilt_north();
            map.tilt_west();
            map.tilt_south();
            map.tilt_east();
        }

        let count = (1000000000 - loop_start) % loop_len;

        for _ in 0..count {
            map.tilt_north();
            map.tilt_west();
            map.tilt_south();
            map.tilt_east();
        }

        Ok(map.load())
    }

    trait Tilter {
        fn tilt_north(&mut self);
        fn tilt_south(&mut self);
        fn tilt_west(&mut self);
        fn tilt_east(&mut self);
    }

    impl Tilter for Map {
        fn tilt_north(&mut self) {
            for i in 0..self.cols {
                self.set_col(i, tilt(&self.col_at(i), false));
            }
        }

        fn tilt_south(&mut self) {
            for i in 0..self.cols {
                self.set_col(i, tilt(&self.col_at(i), true));
            }
        }

        fn tilt_west(&mut self) {
            for i in 0..self.rows {
                self.set_row(i, tilt(self.row_at(i), false));
            }
        }

        fn tilt_east(&mut self) {
            for i in 0..self.rows {
                self.set_row(i, tilt(self.row_at(i), true));
            }
        }
    }

    fn tilt(v: &Vec<char>, rev: bool) -> Vec<char> {
        let mut res = v.clone();
        let mut stop: usize = if rev { v.len() - 1 } else { 0 };

        for i in 0..res.len() {
            let ind = if rev { v.len() - 1 - i } else { i };
            let c = res[ind];
            match c {
                'O' => {
                    res.swap(ind, stop);
                    stop = if rev {
                        if stop > 0 {
                            stop - 1
                        } else {
                            0
                        }
                    } else {
                        stop + 1
                    };
                }
                '.' => continue,
                '#' => {
                    stop = if rev {
                        if ind > 0 {
                            ind - 1
                        } else {
                            0
                        }
                    } else {
                        ind + 1
                    }
                }
                _ => todo!(),
            }
        }
        res
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tilt() {
            assert_eq!(
                tilt(&"O.O#.OO...".chars().collect(), false),
                "OO.#OO....".chars().collect::<Vec<char>>()
            );

            assert_eq!(
                tilt(&"O.O#.OO...".chars().collect(), true),
                ".OO#....OO".chars().collect::<Vec<char>>()
            );
        }
    }
}

mod part1 {
    use super::*;

    pub fn solve(map: &Map) -> Result<usize, Error> {
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

        Ok(res)
    }
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

    fn set_row(&mut self, index: usize, row: Vec<char>) {
        for i in 0..self.cols {
            self.data[index][i] = row[i]
        }
    }

    fn set_col(&mut self, index: usize, col: Vec<char>) {
        for (i, row) in self.data.iter_mut().enumerate() {
            row[index] = col[i]
        }
    }

    fn row_at(&self, index: usize) -> &Vec<char> {
        &self.data[index]
    }

    fn col_at(&self, index: usize) -> Vec<char> {
        let mut res: Vec<char> = Vec::with_capacity(self.rows);
        for row in self.data.iter() {
            res.push(row[index])
        }
        res
    }

    fn load(&self) -> usize {
        let mut res = 0;
        for (i, row) in self.rows().enumerate() {
            res += (self.rows - i) * row.iter().filter(|&&c| c == 'O').count()
        }
        res
    }

    fn hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for vec in &self.data {
            for c in vec {
                c.hash(&mut hasher);
            }
        }
        hasher.finish()
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
