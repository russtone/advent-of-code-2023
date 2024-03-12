use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    thread,
    time::Duration,
};

type Result<T> = std::result::Result<T, &'static str>;

fn main() -> Result<()> {
    let file = File::open("files/input.txt").map_err(|_| "can't open file")?;
    let lines = BufReader::new(file).lines();

    let map = Map::new(lines.map(|line| line.unwrap().chars().collect()).collect());
    let mut max_id = 0;

    let mut next_id = || {
        max_id += 1;
        max_id
    };

    let mut came_from: HashMap<Node, Node> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Node::new(map.start, 0));

    while let Some(cur_node) = queue.pop() {
        let cur_p = cur_node.point;

        if cur_p == map.end {
            break;
        }

        let mut i = 0;
        for next_p in map.get_next(&cur_p).iter() {
            // Don't go back.
            if came_from
                .get(&cur_node)
                .is_some_and(|prev_node| prev_node.point == *next_p)
            {
                continue;
            }

            let id = if i == 0 { cur_node.id } else { next_id() };

            let next_node = Node::new(*next_p, id);

            came_from.insert(next_node, cur_node);

            if *next_p != map.end {
                queue.push(next_node);
            }

            i += 1;
        }
    }

    let res = (0..=max_id)
        .into_iter()
        .map(|id| reconstruct_path(&came_from, &Node::new(map.end, id)).len() - 1)
        .max()
        .unwrap();

    println!("{:?}", res);

    Ok(())
}

fn reconstruct_path(came_from: &HashMap<Node, Node>, start: &Node) -> Vec<Node> {
    let mut path = vec![*start];
    let mut current = start.clone();

    while let Some(n) = came_from.get(&current) {
        current = *n;
        path.push(*n)
    }

    path.reverse();

    path
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    start: Point,
    end: Point,
}

impl Map {
    fn new(data: Vec<Vec<char>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let start = Point::new(0, data[0].iter().position(|&c| c == '.').unwrap());
        let end = Point::new(
            rows - 1,
            data[rows - 1].iter().position(|&c| c == '.').unwrap(),
        );

        Self {
            data,
            rows,
            cols,
            start,
            end,
        }
    }

    fn up(&self, p: &Point) -> Option<Point> {
        if p.row == 0 {
            return None;
        }
        let c = self.data[p.row - 1][p.col];

        if c == '.' || c == '^' {
            return Some(Point::new(p.row - 1, p.col));
        }

        None
    }

    fn down(&self, p: &Point) -> Option<Point> {
        if p.row == self.rows - 1 {
            return None;
        }
        let c = self.data[p.row + 1][p.col];

        if c == '.' || c == 'v' {
            return Some(Point::new(p.row + 1, p.col));
        }

        None
    }

    fn left(&self, p: &Point) -> Option<Point> {
        if p.col == 0 {
            return None;
        }
        let c = self.data[p.row][p.col - 1];

        if c == '.' || c == '<' {
            return Some(Point::new(p.row, p.col - 1));
        }

        None
    }

    fn right(&self, p: &Point) -> Option<Point> {
        if p.col == self.cols - 1 {
            return None;
        }
        let c = self.data[p.row][p.col + 1];

        if c == '.' || c == '>' {
            return Some(Point::new(p.row, p.col + 1));
        }

        None
    }

    fn get_next(&self, p: &Point) -> Vec<Point> {
        let mut res = Vec::new();

        if let Some(p) = self.up(p) {
            res.push(p);
        }
        if let Some(p) = self.down(p) {
            res.push(p);
        }
        if let Some(p) = self.left(p) {
            res.push(p);
        }
        if let Some(p) = self.right(p) {
            res.push(p);
        }

        res
    }

    fn show(&self, came_from: &HashMap<Node, Node>, start: &Node) {
        let path = reconstruct_path(&came_from, start);

        println!();
        for i in 0..self.rows {
            for j in 0..self.cols {
                if let Some(n) = path.iter().find(|n| n.point.row == i && n.point.col == j) {
                    print!("{}", n.id);
                } else {
                    print!("{}", self.data[i][j]);
                }
            }
            print!("\n");
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    id: usize,
    point: Point,
}

impl Node {
    fn new(point: Point, id: usize) -> Self {
        Self { point, id }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.id.cmp(&self.id)
    }
}
