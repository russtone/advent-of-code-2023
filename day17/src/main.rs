use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let file = File::open("files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let map: Map = lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>()
        .into();

    println!("Part 1: {}", solve(&map, 1, 3));
    println!("Part 2: {}", solve(&map, 4, 10));

    Ok(())
}

fn solve(map: &Map, min_moves: u32, max_moves: u32) -> u32 {
    let start = Point::new(0, 0, Direction::Right, max_moves);
    let mut queue = BinaryHeap::new();
    queue.push(Node::new(start, 0));

    let mut g_scores: HashMap<Point, u32> = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores: HashMap<Point, u32> = HashMap::new();
    f_scores.insert(start, 0);

    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let mut path: Vec<Point> = Vec::new();

    let mut seen: HashSet<Point> = HashSet::new();

    while queue.len() > 0 {
        if let Some(n) = queue.pop() {
            let p = n.point;

            if p.row == map.rows - 1 && p.col == map.cols - 1 {
                // show_map(&map, &came_from, &p);
                path = reconstruct_path(&came_from, &p);
                break;
            }

            for np in get_next(&map, &p, min_moves, max_moves).iter() {
                let new_gs = g_scores.get(&p).unwrap() + map.data[np.row][np.col];
                let old_gs = *g_scores.get(&np).unwrap_or(&u32::MAX);

                if new_gs < old_gs {
                    came_from.insert(*np, p);

                    g_scores.insert(*np, new_gs);
                    let fs = new_gs + h(&map, np);
                    f_scores.insert(*np, fs);

                    if !seen.contains(np) {
                        seen.insert(*np);
                        queue.push(Node::new(*np, fs))
                    }
                }
            }
        }
    }

    path.iter()
        .skip(1)
        .map(|p| map.data[p.row][p.col])
        .sum::<u32>()
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    point: Point,
    f_score: u32,
}

impl Node {
    fn new(point: Point, f_score: u32) -> Self {
        Node { point, f_score }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| self.point.cmp(&other.point))
    }
}

fn show_map(map: &Map, came_from: &HashMap<Point, Point>, start: &Point) {
    let path = reconstruct_path(&came_from, start);

    println!();
    for i in 0..map.rows {
        for j in 0..map.cols {
            if let Some(p) = path.iter().find(|p| p.row == i && p.col == j) {
                print!("{}", p.direction);
            } else {
                print!("{}", map.data[i][j]);
            }
        }
        print!("\n");
    }
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, start: &Point) -> Vec<Point> {
    let mut path = vec![*start];
    let mut current = start.clone();

    while let Some(p) = came_from.get(&current) {
        current = *p;
        path.push(*p)
    }

    path.reverse();

    path
}

fn h(map: &Map, p: &Point) -> u32 {
    (map.rows - p.row) as u32 + (map.cols - p.col) as u32
}

fn get_next(map: &Map, point: &Point, min_moves: u32, max_moves: u32) -> Vec<Point> {
    let mut res = Vec::new();

    if let Some(p) = continue_straight(map, point) {
        res.push(p)
    }
    if let Some(p) = turn_left(map, point, min_moves, max_moves) {
        res.push(p)
    }
    if let Some(p) = turn_right(map, point, min_moves, max_moves) {
        res.push(p)
    }

    res
}

fn continue_straight(map: &Map, point: &Point) -> Option<Point> {
    if point.moves_left == 0 {
        return None;
    }

    let mut p = Point::new(point.row, point.col, point.direction, point.moves_left - 1);

    if do_move(map, &mut p) {
        Some(p)
    } else {
        None
    }
}

fn turn_left(map: &Map, point: &Point, min_moves: u32, max_moves: u32) -> Option<Point> {
    if (max_moves - point.moves_left) < min_moves {
        return None;
    }

    let mut p = Point::new(
        point.row,
        point.col,
        point.direction.turn_left(),
        max_moves - 1,
    );

    if do_move(map, &mut p) {
        Some(p)
    } else {
        None
    }
}

fn turn_right(map: &Map, point: &Point, min_moves: u32, max_moves: u32) -> Option<Point> {
    if (max_moves - point.moves_left) < min_moves {
        return None;
    }

    let mut p = Point::new(
        point.row,
        point.col,
        point.direction.turn_right(),
        max_moves - 1,
    );

    if do_move(map, &mut p) {
        Some(p)
    } else {
        None
    }
}

fn do_move(map: &Map, point: &mut Point) -> bool {
    match point.direction {
        Direction::Up => {
            if point.row > 0 {
                point.row -= 1;
                true
            } else {
                false
            }
        }
        Direction::Down => {
            if point.row < map.rows - 1 {
                point.row += 1;
                true
            } else {
                false
            }
        }
        Direction::Left => {
            if point.col > 0 {
                point.col -= 1;
                true
            } else {
                false
            }
        }
        Direction::Right => {
            if point.col < map.cols - 1 {
                point.col += 1;
                true
            } else {
                false
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl From<Vec<Vec<u32>>> for Map {
    fn from(value: Vec<Vec<u32>>) -> Self {
        Map {
            data: value.clone(),
            rows: value.len(),
            cols: value[0].len(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "V"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Ord)]
struct Point {
    row: usize,
    col: usize,
    direction: Direction,
    moves_left: u32,
}

impl Point {
    fn new(row: usize, col: usize, direction: Direction, moves_left: u32) -> Self {
        Point {
            row,
            col,
            direction,
            moves_left,
        }
    }
}
