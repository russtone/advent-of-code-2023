use std::{
    cmp::{max, Ordering},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap,  VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

type Result<T> = std::result::Result<T, &'static str>;

fn main() -> Result<()> {
    let file = File::open("files/input.txt").map_err(|_| "can't open file")?;
    let lines = BufReader::new(file).lines();

    let map = Map::new(lines.map(|line| line.unwrap().chars().collect()).collect());

    println!("Part1: {:?}", part1(&map));
    println!("Part2: {:?}", part2(&map));

    Ok(())
}

fn part2(map: &Map) -> usize {
    let mut edges = map.edges();

    loop {
        let mut to_modify = None;

        for (&p, neighbors) in edges.iter() {
            if neighbors.len() == 2 {
                let mut iter = neighbors.iter();
                let e1 = iter.next().unwrap().clone();
                let e2 = iter.next().unwrap().clone();
                to_modify = Some((p, e1, e2));
                break;
            }
        }

        if let Some((p, e1, e2)) = to_modify {
            if let Some(s) = edges.get_mut(&e1.0) {
                s.remove(&(p, e1.1));
                s.insert((e2.0, e1.1 + e2.1));
            }
            if let Some(s) = edges.get_mut(&e2.0) {
                s.remove(&(p, e2.1));
                s.insert((e1.0, e1.1 + e2.1));
            }
            edges.remove(&p);
        } else {
            break;
        }
    }

    let mut queue = VecDeque::new();
    let mut seen: BTreeSet<Point> = BTreeSet::new();
    let mut res = 0;

    queue.push_back((map.start, 0));

    while let Some((p, d)) = queue.pop_back() {
        if d == 0 && p != map.start {
            seen.remove(&p);
            continue;
        }

        if p == map.end {
            res = max(res, d);
            continue;
        }

        if seen.contains(&p) {
            continue;
        }

        seen.insert(p);
        queue.push_back((p, 0));

        if let Some(s) = edges.get(&p) {
            s.iter().for_each(|e| {
                queue.push_back((e.0, e.1 + d));
            });
        }
    }

    res
}

fn part1(map: &Map) -> usize {
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
        for next_p in map.get_next(&cur_p, true).iter() {
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

    (0..=max_id)
        .into_iter()
        .map(|id| reconstruct_path(&came_from, &Node::new(map.end, id)).len() - 1)
        .max()
        .unwrap()
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

    fn up(&self, p: &Point, slopes: bool) -> Option<Point> {
        if p.row == 0 {
            return None;
        }
        let c = self.data[p.row - 1][p.col];

        if slopes {
            if c == '.' || c == '^' {
                return Some(Point::new(p.row - 1, p.col));
            }
        } else {
            if c != '#' {
                return Some(Point::new(p.row - 1, p.col));
            }
        }

        None
    }

    fn down(&self, p: &Point, slopes: bool) -> Option<Point> {
        if p.row == self.rows - 1 {
            return None;
        }
        let c = self.data[p.row + 1][p.col];

        if slopes {
            if c == '.' || c == 'v' {
                return Some(Point::new(p.row + 1, p.col));
            }
        } else {
            if c != '#' {
                return Some(Point::new(p.row + 1, p.col));
            }
        }

        None
    }

    fn left(&self, p: &Point, slopes: bool) -> Option<Point> {
        if p.col == 0 {
            return None;
        }
        let c = self.data[p.row][p.col - 1];

        if slopes {
            if c == '.' || c == '<' {
                return Some(Point::new(p.row, p.col - 1));
            }
        } else {
            if c != '#' {
                return Some(Point::new(p.row, p.col - 1));
            }
        }

        None
    }

    fn right(&self, p: &Point, slopes: bool) -> Option<Point> {
        if p.col == self.cols - 1 {
            return None;
        }
        let c = self.data[p.row][p.col + 1];

        if slopes {
            if c == '.' || c == '>' {
                return Some(Point::new(p.row, p.col + 1));
            }
        } else {
            if c != '#' {
                return Some(Point::new(p.row, p.col + 1));
            }
        }

        None
    }

    fn get_next(&self, p: &Point, slopes: bool) -> Vec<Point> {
        let mut res = Vec::new();

        if let Some(p) = self.up(p, slopes) {
            res.push(p);
        }
        if let Some(p) = self.down(p, slopes) {
            res.push(p);
        }
        if let Some(p) = self.left(p, slopes) {
            res.push(p);
        }
        if let Some(p) = self.right(p, slopes) {
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

    fn edges(&self) -> BTreeMap<Point, BTreeSet<(Point, usize)>> {
        let mut edges: BTreeMap<Point, BTreeSet<(Point, usize)>> = BTreeMap::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let ch = self.data[row][col];
                if ch == '.' || ch == '>' || ch == 'v' {
                    let p = Point::new(row, col);
                    for np in self.get_next(&p, false).iter() {
                        edges
                            .entry(p)
                            .and_modify(|s| {
                                s.insert((*np, 1));
                            })
                            .or_insert(BTreeSet::from_iter(vec![(*np, 1)]));

                        edges
                            .entry(*np)
                            .and_modify(|s| {
                                s.insert((p, 1));
                            })
                            .or_insert(BTreeSet::from_iter(vec![(p, 1)]));
                    }
                }
            }
        }

        edges
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
