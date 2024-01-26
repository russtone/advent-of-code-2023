use std::{
    collections::{HashSet, VecDeque},
    error,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let file = File::open("files/input.txt")?;
    let lines = BufReader::new(file).lines();

    let map: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();

    println!(
        "Part 1: {}",
        count_energized(&map, &Beam::new(0, 0, Direction::Right))
    );

    println!(
        "Part 2: {}",
        entry_points(&map)
            .iter()
            .map(|beam| count_energized(&map, beam))
            .max()
            .unwrap()
    );

    Ok(())
}

fn entry_points(map: &Vec<Vec<char>>) -> Vec<Beam> {
    let mut res = Vec::new();
    let rows = map.len();
    let cols = map[0].len();

    for col in 0..cols {
        res.push(Beam::new(0, col as isize, Direction::Down));
        res.push(Beam::new((rows - 1) as isize, col as isize, Direction::Up))
    }

    for row in 0..rows {
        res.push(Beam::new(row as isize, 0, Direction::Right));
        res.push(Beam::new(
            row as isize,
            (cols - 1) as isize,
            Direction::Right,
        ));
    }

    res
}

fn count_energized(map: &Vec<Vec<char>>, start: &Beam) -> usize {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    let mut seen: HashSet<Beam> = HashSet::new();
    let mut beams: VecDeque<Beam> = vec![*start].into();

    while let Some(mut beam) = beams.pop_front() {
        if seen.contains(&beam) {
            continue;
        }
        seen.insert(beam);

        match map[beam.row as usize][beam.col as usize] {
            '.' => {}
            '\\' => match beam.direction {
                Direction::Up => beam.direction = Direction::Left,
                Direction::Down => beam.direction = Direction::Right,
                Direction::Left => beam.direction = Direction::Up,
                Direction::Right => beam.direction = Direction::Down,
            },
            '/' => match beam.direction {
                Direction::Up => beam.direction = Direction::Right,
                Direction::Down => beam.direction = Direction::Left,
                Direction::Left => beam.direction = Direction::Down,
                Direction::Right => beam.direction = Direction::Up,
            },
            '-' => match beam.direction {
                Direction::Left | Direction::Right => {}
                Direction::Up | Direction::Down => {
                    beam.direction = Direction::Left;
                    beams.push_back(Beam::new(beam.row, beam.col, Direction::Right));
                }
            },
            '|' => match beam.direction {
                Direction::Left | Direction::Right => {
                    beam.direction = Direction::Up;
                    beams.push_back(Beam::new(beam.row, beam.col, Direction::Down));
                }
                Direction::Up | Direction::Down => {}
            },
            _ => todo!(),
        }

        match beam.direction {
            Direction::Up => beam.row -= 1,
            Direction::Down => beam.row += 1,
            Direction::Left => beam.col -= 1,
            Direction::Right => beam.col += 1,
        }

        if beam.row >= 0 && beam.row < rows && beam.col >= 0 && beam.col < cols {
            beams.push_back(beam)
        }
    }

    let energized: HashSet<(isize, isize)> = seen.iter().map(|beam| (beam.row, beam.col)).collect();

    energized.len()
}

fn print_map(map: &Vec<Vec<char>>, energized: &HashSet<(isize, isize)>) {
    println!();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if energized.contains(&(i as isize, j as isize)) {
                print!("#");
            } else {
                print!("{}", map[i][j]);
            }
        }
        print!("\n");
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Beam {
    row: isize,
    col: isize,
    direction: Direction,
}

impl Beam {
    fn new(row: isize, col: isize, direction: Direction) -> Self {
        Beam {
            row,
            col,
            direction,
        }
    }
}
