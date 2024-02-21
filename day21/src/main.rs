use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read},
};

type Result<T> = std::result::Result<T, &'static str>;

fn main() -> Result<()> {
    let mut file = File::open("files/input.txt").map_err(|_| "can't open file")?;

    let map = parse(&mut file)?;
    let start = map.find_start().ok_or("no start")?;

    println!("Part 1: {:?}", part1::solve(&map, start));
    println!("Part 2: {:?}", part2::solve(&map, start),);

    Ok(())
}

/*
┌─────┬─────┬──╳──┬─────┬─────┐
│#####│*****│#╱#╲#│*****│#####│
│#####│*****│╱###╲│*****│#####│
│#####│*****╳#####╳*****│#####│
│#####│****╱│#####│╲****│#####│
│#####│***╱*│#####│*╲***│#####│
├─────┼──╳──┼─────┼──╳──┼─────┤
│*****│#╱###│*****│###╲#│*****│
│*****│╱####│*****│####╲│*****│
│*****╳#####│*****│#####╳*****│
│****╱│#####│*****│#####│╲****│
│***╱*│#####│*****│#####│*╲***│
├──╳──┼─────┼─────┼─────┼──╳──┤
│#╱###│*****│#####│*****│###╲#│
│╱####│*****│#####│*****│####╲│
╲#####│*****│#####│*****│#####▲
│╲####│*****│#####│*****│####╱│
│#╲###│*****│#####│*****│###╱#│
├──╳──┼─────┼─────┼─────┼──╳──┤
│***╲*│#####│*****│#####│*╱***│
│****╲│#####│*****│#####│╱****│
│*****╳#####│*****│#####╳*****│
│*****│╲####│*****│####╱│*****│
│*****│#╲###│*****│###╱#│*****│
├─────┼──╳──┼─────┼──╳──┼─────┤
│#####│***╲*│#####│*╱***│#####│
│#####│****╲│#####│╱****│#####│
│#####│*****╳#####╳*****│#####│
│#####│*****│╲###╱│*****│#####│
│#####│*****│#╲#╱#│*****│#####│
└─────┴─────┴──╱──┴─────┴─────┘
*/

mod part2 {

    use super::*;

    pub fn solve(map: &Map, start: (usize, usize)) -> usize {
        let mut points: VecDeque<((usize, usize), usize)> = VecDeque::new();
        let mut seen: HashMap<(usize, usize), usize> = HashMap::new();
        points.push_back((start, 0));

        while let Some(((row, col), dist)) = points.pop_front() {
            if seen.contains_key(&(row, col)) {
                continue;
            }
            seen.insert((row, col), dist);
            if row > 0 && get_at(map, row - 1, col) == Some('.') {
                points.push_back(((row - 1, col), dist + 1))
            }
            if get_at(map, row + 1, col) == Some('.') {
                points.push_back(((row + 1, col), dist + 1))
            }
            if col > 0 && get_at(map, row, col - 1) == Some('.') {
                points.push_back(((row, col - 1), dist + 1))
            }
            if get_at(map, row, col + 1) == Some('.') {
                points.push_back(((row, col + 1), dist + 1))
            }
        }

        let even = seen.values().filter(|d| *d % 2 == 0).count();
        let odd = seen.values().filter(|d| *d % 2 == 1).count();

        let half = map.cols / 2;
        assert!(half == 65);

        assert!((26501365 - half) % map.cols == 0);

        let n = (26501365 - half) / map.cols;

        let even_corners = seen
            .iter()
            .filter(|&(&(row, col), dist)| {
                dist % 2 == 0 && half.abs_diff(row) + half.abs_diff(col) > 65
            })
            .count();

        let odd_corners = seen
            .iter()
            .filter(|&(&(row, col), dist)| {
                dist % 2 == 1 && half.abs_diff(row) + half.abs_diff(col) > 65
            })
            .count();

        let res = (n + 1) * (n + 1) * odd + n * n * even - (n + 1) * odd_corners + n * even_corners;

        res
    }
}

mod part1 {
    use super::*;

    pub fn solve(map: &Map, start: (usize, usize)) -> usize {
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
                    if row > 0 && get_at(map, row - 1, col) == Some('.') {
                        points.push_back((row - 1, col))
                    }
                    if get_at(map, row + 1, col) == Some('.') {
                        points.push_back((row + 1, col))
                    }
                    if col > 0 && get_at(map, row, col - 1) == Some('.') {
                        points.push_back((row, col - 1))
                    }
                    if get_at(map, row, col + 1) == Some('.') {
                        points.push_back((row, col + 1))
                    }
                }
            }
        }

        seen.len()
    }
}

fn get_at(map: &Map, row: usize, col: usize) -> Option<char> {
    if row > map.rows - 1 || col > map.cols - 1 {
        return None;
    }

    if map.data[row][col] == 'S' {
        return Some('.');
    }

    Some(map.data[row][col])
}

fn show_map(map: &Map, seen: &HashMap<(usize, usize), usize>) {
    println!();
    for i in 0..map.rows {
        for j in 0..map.cols {
            if seen.contains_key(&(i, j)) {
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
}
