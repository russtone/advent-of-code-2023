use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calc(num_fn: fn(&Num) -> u32, sym_fn: fn(&Sym) -> u32) -> u32 {
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
                        match sym.first_two_adjacent_nums {
                            (None, None) => {
                                sym.first_two_adjacent_nums = (Some(num.clone()), None);
                                sym.adjacent_nums_count += 1;
                            }
                            (Some(n1), None) => {
                                if num.pos != n1.pos {
                                    sym.first_two_adjacent_nums = (Some(n1), Some(num.clone()));
                                    sym.adjacent_nums_count += 1;
                                }
                            }
                            (Some(n1), Some(n2)) => {
                                if num.pos != n1.pos && num.pos != n2.pos {
                                    sym.adjacent_nums_count += 1;
                                }
                            }
                            _ => panic!("must never happen"),
                        }
                        num.has_adjacent_sym = true;
                    }
                }
            }

            for num in nums
                .iter()
                .filter(|n| (n.pos.row == (i - 2) || lines.peek().is_none()))
            {
                sum += num_fn(num)
            }
            nums.retain(|n| n.pos.row > (i - 2));

            for sym in syms
                .iter()
                .filter(|s| (s.pos.row == (i - 2) || lines.peek().is_none()))
            {
                sum += sym_fn(sym)
            }
            syms.retain(|s| s.pos.row > (i - 2));
        }
    }

    return sum;
}

fn main() {
    println!(
        "Part 1: {}",
        calc(|n| if n.has_adjacent_sym { n.value } else { 0 }, |_| 0)
    );
    println!(
        "Part 2: {}",
        calc(
            |_| 0,
            |s| if s.adjacent_nums_count == 2 {
                s.first_two_adjacent_nums.0.unwrap().value
                    * s.first_two_adjacent_nums.1.unwrap().value
            } else {
                0
            }
        )
    )
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Num {
    pos: Pos,
    value: u32,
    has_adjacent_sym: bool,
}

impl Num {
    fn new(pos: Pos, value: u32) -> Num {
        Num {
            pos,
            value,
            has_adjacent_sym: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Sym {
    pos: Pos,
    adjacent_nums_count: u32,
    first_two_adjacent_nums: (Option<Num>, Option<Num>),
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
