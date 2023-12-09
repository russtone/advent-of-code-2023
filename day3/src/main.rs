use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = match File::open("./files/input.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    };

    let mut lines = BufReader::new(file).lines().enumerate().peekable();
    let mut num_pos: Option<Pos>;
    let mut sum: u32 = 0;
    let mut nums: Vec<Num> = Vec::new();
    let mut syms: Vec<Sym> = Vec::new();

    while let Some((i, line)) = lines.next() {
        let line = line.unwrap();
        num_pos = None;

        for (j, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => match num_pos {
                    Some(ref mut pos) => pos.end = j + 1,
                    None => num_pos = Some(Pos::new(i, j, j + 1)),
                },
                _ => {
                    match c {
                        '.' => {}
                        _ => syms.push(Sym::new(i, j)),
                    }
                    parse_and_save(&line, &num_pos, &mut nums);
                    num_pos = None
                }
            }
        }
        parse_and_save(&line, &num_pos, &mut nums);

        if i >= 2 {
            for num in nums.iter_mut() {
                for sym in syms.iter_mut() {
                    if is_adjacent(sym, num) {
                        sym.adjacent_nums_count += 1;
                        match sym.first_two_adjacent_nums {
                            (None, None) => sym.first_two_adjacent_nums = (Some(num.value), None),
                            (Some(n1), None) => {
                                sym.first_two_adjacent_nums = (Some(n1), Some(num.value))
                            }
                            _ => {}
                        }
                        num.is_inclued = true;
                    }
                }
            }

            for num in nums
                .iter()
                .filter(|n| (n.pos.row == (i - 2) || lines.peek().is_none()) && n.is_inclued)
            {
                sum += num.value;
            }
            nums.retain(|n| n.pos.row > (i - 2));
        }
    }

    println!("Answer: {}", sum)
}

fn parse_and_save(line: &str, pos: &Option<Pos>, nums: &mut Vec<Num>) {
    if let Some(pos) = pos {
        let s = &line[pos.start..pos.end];
        let n: u32 = s.parse().unwrap();
        nums.push(Num::new(*pos, n));
    }
}

fn is_adjacent(sym: &Sym, num: &Num) -> bool {
    sym.pos.row + 1 >= num.pos.row
        && sym.pos.row <= num.pos.row + 1
        && sym.pos.start + 1 >= num.pos.start
        && sym.pos.start <= num.pos.end
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    row: usize,
    start: usize,
    end: usize,
}

impl Pos {
    fn new(row: usize, start: usize, end: usize) -> Pos {
        return Pos { row, start, end };
    }
}

#[derive(Debug, Clone, Copy)]
struct Num {
    pos: Pos,
    value: u32,
    is_inclued: bool,
}

impl Num {
    fn new(pos: Pos, value: u32) -> Num {
        Num {
            pos,
            value,
            is_inclued: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Sym {
    pos: Pos,
    adjacent_nums_count: u32,
    first_two_adjacent_nums: (Option<u32>, Option<u32>),
}

impl Sym {
    fn new(row: usize, col: usize) -> Sym {
        Sym {
            pos: Pos::new(row, col, col),
            adjacent_nums_count: 0,
            first_two_adjacent_nums: (None, None),
        }
    }
}
