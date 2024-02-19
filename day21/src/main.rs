use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read},
};

type Result<T> = std::result::Result<T, &'static str>;

fn main() -> Result<()> {
    let mut file = File::open("files/input.txt").map_err(|_| "can't open file")?;

    let map = parse(&mut file)?;
    let start = map.find_start().ok_or("no start")?;
    let mut points: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    points.push_back(start);

    for _ in 0..=64 {
        seen.clear();
        let mut count = points.len();

        while count > 0 {
            count -= 1;
            if let Some((row, col)) = points.pop_front() {
                if !seen.insert((row, col)) {
                    continue;
                }
                if row > 0 && map.get_at(row - 1, col) == Some('.') {
                    points.push_back((row - 1, col))
                }
                if map.get_at(row + 1, col) == Some('.') {
                    points.push_back((row + 1, col))
                }
                if col > 0 && map.get_at(row, col - 1) == Some('.') {
                    points.push_back((row, col - 1))
                }
                if map.get_at(row, col + 1) == Some('.') {
                    points.push_back((row, col + 1))
                }
            }
        }
    }

    println!("{:?}", seen.len());

    Ok(())
}

fn show_map(map: &Map, seen: &HashSet<(usize, usize)>) {
    println!();
    for i in 0..map.rows {
        for j in 0..map.cols {
            if seen.contains(&(i, j)) {
                print!("O");
            } else {
                print!("{}", map.data[i][j]);
            }
        }
        print!("\n");
    }
}

fn parse<R: Read>(buf: &mut R) -> Result<Map> {
    let lines = BufReader::new(buf).lines();
    let data: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();
    let rows = data.len();
    let cols = data[0].len();
    Ok(Map { data, rows, cols })
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn find_start(&self) -> Option<(usize, usize)> {
        for (row, line) in self.data.iter().enumerate() {
            for (col, ch) in line.iter().enumerate() {
                if ch == &'S' {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn get_at(&self, row: usize, col: usize) -> Option<char> {
        if row > self.rows - 1 || col > self.cols - 1 {
            return None;
        }

        if self.data[row][col] == 'S' {
            return Some('.');
        }

        Some(self.data[row][col])
    }
}
