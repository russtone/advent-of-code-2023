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

    let lines = BufReader::new(file).lines();
    let mut pos: Option<Pos>;
    let mut sum: u32 = 0;
    let mut last_line_num: Vec<Num> = Vec::new();
    let mut this_line_num: Vec<Num> = Vec::new();
    let mut last_line_sym: Vec<usize> = Vec::new();
    let mut this_line_sym: Vec<usize> = Vec::new();
    let mut last_sym: Option<usize>;

    for line in lines {
        let line = line.unwrap();
        pos = None;
        last_sym = None;

        for (j, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => match pos {
                    Some(ref mut pos) => pos.end = j + 1,
                    None => pos = Some(Pos::new(j, j + 1)),
                },
                _ => {
                    match c {
                        '.' => {}
                        _ => {
                            this_line_sym.push(j);
                            last_sym = Some(j);
                        }
                    }
                    if let Some(p) = pos {
                        let s = &line[p.start..p.end];
                        let n: u32 = s.parse().unwrap();
                        if last_sym.is_some() && is_near(&last_sym.unwrap(), &p) {
                            sum += n
                        } else {
                            this_line_num.push(Num::new(p, n));
                        }
                        pos = None
                    }
                }
            }
        }

        if let Some(p) = pos {
            let s = &line[p.start..p.end];
            let n: u32 = s.parse().unwrap();
            if last_sym.is_some() && is_near(&last_sym.unwrap(), &p) {
                sum += n
            } else {
                this_line_num.push(Num::new(p, n));
            }
        }

        'outer: for num in last_line_num.iter() {
            for sym in this_line_sym.iter() {
                if is_near(sym, &num.pos) {
                    sum += num.value;
                    continue 'outer;
                }
            }
        }

        last_line_num.clear();

        'outer: for num in this_line_num.iter() {
            for sym in last_line_sym.iter() {
                if is_near(sym, &num.pos) {
                    sum += num.value;
                    continue 'outer;
                }
            }
            last_line_num.push(*num)
        }

        last_line_sym = this_line_sym.to_owned();
        this_line_num.clear();
        this_line_sym.clear();
    }

    println!("Answer: {}", sum)
}

fn is_near(i: &usize, pos: &Pos) -> bool {
    *i + 1 >= pos.start && *i <= pos.end
}

#[derive(Debug, Clone, Copy)]
struct Num {
    pos: Pos,
    value: u32,
}

impl Num {
    fn new(pos: Pos, value: u32) -> Num {
        Num { pos, value }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    start: usize,
    end: usize,
}

impl Pos {
    fn new(start: usize, end: usize) -> Pos {
        Pos { start, end }
    }
}
