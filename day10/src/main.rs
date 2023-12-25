use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let lines: Vec<String> = BufReader::new(file).lines().collect::<Result<_, _>>()?;

    let map = Map::new(lines);

    let mut queue: BTreeSet<Point> = BTreeSet::new();
    let mut seen: BTreeSet<Point> = BTreeSet::new();
    let mut steps: usize = 0;
    queue.insert(map.find_start());

    loop {
        let mut new_points = Vec::new();

        while let Some(p) = queue.pop_first() {
            let connected = map.find_connected(&p);
            seen.insert(p);
            for q in &connected {
                if !seen.contains(q) {
                    new_points.push(*q);
                }
            }
        }

        if new_points.is_empty() {
            break;
        }

        queue.extend(new_points.clone());
        steps += 1;
    }

    println!("Steps: {}", steps);

    let mut count = 0;
    for p in MapIterator::new(&map) {
        if map.is_inside(&seen, &p) {
            count += 1;
        }
    }

    println!("Count: {}", count);

    Ok(())
}

#[derive(Debug)]
struct Map {
    data: Vec<String>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(data: Vec<String>) -> Map {
        let rows = data.len();
        let cols = data.first().unwrap().len();
        Map { data, rows, cols }
    }

    fn find_start(&self) -> Point {
        let mut p = Point::new(0, 0, 'S');
        for (i, line) in self.data.iter().enumerate() {
            if let Some(j) = line.find("S") {
                p.row = i;
                p.col = j;
            }
        }
        p
    }

    fn find_connected(&self, p: &Point) -> Vec<Point> {
        let mut res: Vec<Point> = Vec::new();

        if p.col > 0 {
            if let Some(c) = self.get_char_at(p.row, p.col - 1) {
                match (p.char, c) {
                    ('S' | '-' | 'J' | '7', '-' | 'F' | 'L') => {
                        res.push(Point::new(p.row, p.col - 1, c))
                    }
                    _ => {}
                }
            }
        }
        if p.col < self.cols - 1 {
            if let Some(c) = self.get_char_at(p.row, p.col + 1) {
                match (p.char, c) {
                    ('S' | '-' | 'F' | 'L', '-' | '7' | 'J') => {
                        res.push(Point::new(p.row, p.col + 1, c))
                    }
                    _ => {}
                }
            }
        }
        if p.row > 0 {
            if let Some(c) = self.get_char_at(p.row - 1, p.col) {
                match (p.char, c) {
                    ('S' | '|' | 'L' | 'J', '|' | '7' | 'F') => {
                        res.push(Point::new(p.row - 1, p.col, c))
                    }
                    _ => {}
                }
            }
        }
        if p.row < self.rows - 1 {
            if let Some(c) = self.get_char_at(p.row + 1, p.col) {
                match (p.char, c) {
                    ('S' | '|' | '7' | 'F', '|' | 'L' | 'J') => {
                        res.push(Point::new(p.row + 1, p.col, c))
                    }
                    _ => {}
                }
            }
        }

        res
    }

    fn get_char_at(&self, row: usize, col: usize) -> Option<char> {
        self.data.get(row)?.chars().nth(col)
    }

    fn get_point_at(&self, row: usize, col: usize) -> Option<Point> {
        Some(Point::new(row, col, self.get_char_at(row, col)?))
    }

    fn is_inside(&self, points: &BTreeSet<Point>, p: &Point) -> bool {
        if points.contains(p) {
            return false;
        }

        let directions = vec![p.col, p.row, self.cols - p.col, self.rows - p.row];
        let m = directions.iter().enumerate().min_by_key(|x| x.1).unwrap();

        let mut n = 0;
        let mut path: Vec<Point> = Vec::new();

        match m {
            (0, _) => {
                for col in 0..p.col {
                    if let Some(q) = self.get_point_at(p.row, col) {
                        if points.contains(&q) {
                            if let Some(r) = path.last() {
                                match (r.char, q.char) {
                                    ('S' | '-' | 'F' | 'L', '-' | '7' | 'J') => {
                                        path.push(q);
                                        continue;
                                    }
                                    _ => {
                                        match path.as_slice() {
                                            [first, .., last] => match (first.char, last.char) {
                                                ('F', '7') | ('L', 'J') => n += 2,
                                                ('L', '7') | ('F', 'J') => n += 1,
                                                _ => panic!("error"),
                                            },
                                            [_] => n += 1,
                                            _ => {}
                                        }
                                        path.clear();
                                        path.push(q);
                                        continue;
                                    }
                                }
                            } else {
                                path.push(q)
                            }
                        }
                    }
                }
                match path.as_slice() {
                    [first, .., last] => match (first.char, last.char) {
                        ('F', '7') | ('L', 'J') => n += 2,
                        ('L', '7') | ('F', 'J') => n += 1,
                        _ => panic!("error"),
                    },
                    [_] => n += 1,
                    _ => {}
                }
            }
            (1, _) => {
                for row in 0..p.row {
                    if let Some(q) = self.get_point_at(row, p.col) {
                        if points.contains(&q) {
                            if let Some(r) = path.last() {
                                match (r.char, q.char) {
                                    ('S' | '|' | '7' | 'F', '|' | 'L' | 'J') => {
                                        path.push(q);
                                        continue;
                                    }
                                    _ => {
                                        match path.as_slice() {
                                            [first, .., last] => match (first.char, last.char) {
                                                ('F', 'L') | ('7', 'J') => n += 2,
                                                ('F', 'J') | ('7', 'L') => n += 1,
                                                _ => panic!("error"),
                                            },
                                            [_] => n += 1,
                                            _ => {}
                                        }
                                        path.clear();
                                        path.push(q);
                                        continue;
                                    }
                                }
                            } else {
                                path.push(q);
                            }
                        }
                    }
                }
                match path.as_slice() {
                    [first, .., last] => match (first.char, last.char) {
                        ('F', 'L') | ('7', 'J') => n += 2,
                        ('F', 'J') | ('7', 'L') => n += 1,
                        _ => panic!("error"),
                    },
                    [_] => n += 1,
                    _ => {}
                }
            }
            (2, _) => {
                path.clear();
                for col in p.col + 1..self.cols {
                    if let Some(q) = self.get_point_at(p.row, col) {
                        if points.contains(&q) {
                            if let Some(r) = path.last() {
                                match (r.char, q.char) {
                                    ('S' | '-' | 'F' | 'L', '-' | '7' | 'J') => {
                                        path.push(q);
                                        continue;
                                    }
                                    _ => {
                                        match path.as_slice() {
                                            [first, .., last] => match (first.char, last.char) {
                                                ('F', '7') | ('L', 'J') => n += 2,
                                                ('L', '7') | ('F', 'J') => n += 1,
                                                _ => panic!("error"),
                                            },
                                            [_] => n += 1,
                                            _ => {}
                                        }
                                        path.clear();
                                        path.push(q);
                                        continue;
                                    }
                                }
                            } else {
                                path.push(q)
                            }
                        }
                    }
                }
                match path.as_slice() {
                    [first, .., last] => match (first.char, last.char) {
                        ('F', '7') | ('L', 'J') => n += 2,
                        ('L', '7') | ('F', 'J') => n += 1,
                        _ => panic!("error"),
                    },
                    [_] => n += 1,
                    _ => {}
                }
            }
            (3, _) => {
                for row in p.row + 1..self.rows {
                    if let Some(q) = self.get_point_at(row, p.col) {
                        if points.contains(&q) {
                            if let Some(r) = path.last() {
                                match (r.char, q.char) {
                                    ('S' | '|' | '7' | 'F', '|' | 'L' | 'J') => {
                                        path.push(q);
                                        continue;
                                    }
                                    _ => {
                                        match path.as_slice() {
                                            [first, .., last] => match (first.char, last.char) {
                                                ('F', 'L') | ('7', 'J') => n += 2,
                                                ('F', 'J') | ('7', 'L') => n += 1,
                                                _ => panic!("error"),
                                            },
                                            [_] => n += 1,
                                            _ => {}
                                        }
                                        path.clear();
                                        path.push(q);
                                        continue;
                                    }
                                }
                            } else {
                                path.push(q);
                            }
                        }
                    }
                }
                match path.as_slice() {
                    [first, .., last] => match (first.char, last.char) {
                        ('F', 'L') | ('7', 'J') => n += 2,
                        ('F', 'J') | ('7', 'L') => n += 1,
                        _ => panic!("error"),
                    },
                    [_] => n += 1,
                    _ => {}
                }
            }
            (_, _) => panic!("must never happen"),
        }

        n % 2 == 1
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
    char: char,
}

impl Point {
    fn new(row: usize, col: usize, char: char) -> Point {
        Point { row, col, char }
    }
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

#[derive(Debug)]
struct MapIterator<'a> {
    map: &'a Map,
    row: usize,
    col: usize,
}

impl<'a> MapIterator<'a> {
    fn new(map: &'a Map) -> Self {
        MapIterator {
            map,
            row: 0,
            col: 0,
        }
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.map.cols - 1 {
            self.col += 1;
            self.map.get_point_at(self.row, self.col)
        } else {
            if self.row < self.map.rows - 1 {
                self.row += 1;
                self.col = 1;
                self.map.get_point_at(self.row, self.col)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map_str = r#"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
"#;

        let map = Map::new(
            map_str
                .lines()
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
        );

        assert_eq!(map.cols, 5);
        assert_eq!(map.rows, 5);

        assert_eq!(map.find_start(), Point::new(1, 1, 'S'));

        assert_eq!(
            map.find_connected(&map.find_start()),
            vec![Point::new(1, 2, '-'), Point::new(2, 1, '|')],
        );

        assert_eq!(
            map.find_connected(&Point::new(3, 2, '-')),
            vec![Point::new(3, 1, 'L'), Point::new(3, 3, 'J')],
        );
    }
}
