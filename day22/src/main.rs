use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

type Result<T> = std::result::Result<T, &'static str>;

fn main() -> Result<()> {
    let mut file = File::open("files/input.txt").map_err(|_| "can't open file")?;

    let mut bricks = parse(&mut file)?;
    let (settled, supports, supported_by) = drop(&mut bricks);

    println!("Part1: {}", part1(&settled, &supports, &supported_by));

    Ok(())
}

fn part1(
    settled: &BTreeSet<Brick>,
    supports: &BTreeMap<u32, BTreeSet<u32>>,
    supported_by: &BTreeMap<u32, BTreeSet<u32>>,
) -> u64 {
    let mut res = 0;

    for b in settled.iter().rev() {
        if let Some(sup) = supports.get(&b.id) {
            if sup
                .iter()
                .all(|id| supported_by.get(id).is_some_and(|ids| ids.len() > 1))
            {
                res += 1;
            }
        } else {
            res += 1;
        }
    }

    res
}

fn drop(
    bricks: &mut BTreeSet<Brick>,
) -> (
    BTreeSet<Brick>,
    BTreeMap<u32, BTreeSet<u32>>,
    BTreeMap<u32, BTreeSet<u32>>,
) {
    let mut z_edges: BTreeMap<usize, BTreeSet<Edge>> = BTreeMap::new();
    let mut settled: BTreeSet<Brick> = BTreeSet::new();
    let mut supports: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut supported_by: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();

    while let Some(mut brick) = bricks.pop_first() {
        let edge = brick.edge();
        let mut z = brick.first.z;

        loop {
            if z <= 1 {
                break;
            }

            if let Some(intersecting) = z_edges
                .get(&(z - 1))
                .map(|edges| edges.iter().filter(|e| e.intersects(&edge)))
            {
                let mut count = 0;

                intersecting.for_each(|e| {
                    // Current brick is supported by all bricks with intersecting edges.
                    supported_by
                        .entry(brick.id)
                        .and_modify(|entry| {
                            entry.insert(e.id);
                        })
                        .or_insert(BTreeSet::from_iter(vec![e.id]));

                    // Brick with intersecting edge support current brick.
                    supports
                        .entry(e.id)
                        .and_modify(|entry| {
                            entry.insert(brick.id);
                        })
                        .or_insert(BTreeSet::from_iter(vec![brick.id]));

                    count += 1;
                });

                if count > 0 {
                    break;
                }
            }

            z -= 1;
        }

        brick.second.z -= brick.first.z - z;
        brick.first.z = z;

        // Save top edge of settled brick.
        z_edges
            .entry(brick.second.z)
            .and_modify(|e| {
                e.insert(edge);
            })
            .or_insert(BTreeSet::from_iter(vec![edge]));

        settled.insert(brick);
    }

    (settled, supports, supported_by)
}

fn parse<R: Read>(buf: &mut R) -> Result<BTreeSet<Brick>> {
    let lines = BufReader::new(buf).lines();
    let mut bricks: BTreeSet<Brick> = BTreeSet::new();

    let mut id = 0;
    for line in lines {
        let line = line.map_err(|_| "can't get line")?;

        if let Some((left, right)) = line.split_once("~") {
            let l: Vec<usize> = left
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            let r: Vec<usize> = right
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            let first = Coord3::new(l[0], l[1], l[2]);
            let second = Coord3::new(r[0], r[1], r[2]);

            assert!(first.z <= second.z);

            bricks.insert(Brick::new(id, first, second));
            id += 1;
        }
    }

    Ok(bricks)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Brick {
    id: u32,
    first: Coord3,
    second: Coord3,
}

impl Brick {
    fn new(id: u32, first: Coord3, second: Coord3) -> Self {
        Brick { id, first, second }
    }

    fn edge(&self) -> Edge {
        Edge::new(
            self.id,
            Coord2::new(self.first.x, self.first.y),
            Coord2::new(self.second.x, self.second.y),
        )
    }
}
impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first
            .z
            .cmp(&other.first.z)
            .then_with(|| self.id.cmp(&other.id))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Edge {
    id: u32,
    first: Coord2,
    second: Coord2,
}

impl Edge {
    fn new(id: u32, first: Coord2, second: Coord2) -> Self {
        return Self { id, first, second };
    }

    fn kind(&self) -> EdgeKind {
        if self.first.x == self.second.x {
            EdgeKind::YParallel
        } else {
            EdgeKind::XParallel
        }
    }

    fn intersects(&self, other: &Edge) -> bool {
        let (e1, e2) = if self.first > other.first {
            (other, self)
        } else {
            (self, other)
        };

        match (e1.kind(), e2.kind()) {
            (EdgeKind::XParallel, EdgeKind::XParallel) => {
                e1.first.y == e2.first.y && e2.first.x >= e1.first.x && e2.first.x <= e1.second.x
            }
            (EdgeKind::YParallel, EdgeKind::YParallel) => {
                e1.first.x == e2.first.x && e2.first.y >= e1.first.y && e2.first.y <= e1.second.y
            }
            (EdgeKind::XParallel, EdgeKind::YParallel) => {
                e1.first.y >= e2.first.y
                    && e1.first.y <= e2.second.y
                    && e2.first.x >= e1.first.x
                    && e2.first.x <= e1.second.x
            }
            (EdgeKind::YParallel, EdgeKind::XParallel) => {
                e2.first.y >= e1.first.y
                    && e2.first.y <= e1.second.y
                    && e1.first.x >= e2.first.x
                    && e1.first.x <= e2.second.x
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EdgeKind {
    XParallel,
    YParallel,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord2 {
    x: usize,
    y: usize,
}

impl Coord2 {
    fn new(x: usize, y: usize) -> Coord2 {
        Coord2 { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
struct Coord3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord3 {
    fn new(x: usize, y: usize, z: usize) -> Coord3 {
        Coord3 { x, y, z }
    }
}
