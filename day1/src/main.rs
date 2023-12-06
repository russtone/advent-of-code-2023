use std::{fs::File, io::BufRead, io::BufReader};

fn part1() -> Result<u32, String> {
    let file = match File::open("./files/input.txt") {
        Ok(file) => file,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let lines = BufReader::new(file).lines();

    let mut numbers: (Option<u32>, Option<u32>);
    let mut sum: u32 = 0;

    for line in lines {
        numbers = (None, None);

        if let Err(err) = line {
            return Err(err.to_string());
        }

        let line = line.unwrap();

        for c in line.chars() {
            if !c.is_ascii_digit() {
                continue;
            }
            numbers.0 = Some(c.to_digit(10).unwrap());
            break;
        }

        for c in line.chars().rev() {
            if !c.is_ascii_digit() {
                continue;
            }
            numbers.1 = Some(c.to_digit(10).unwrap());
            break;
        }

        sum += 10 * numbers.0.unwrap() + numbers.1.unwrap();
    }
    return Ok(sum);
}

fn part2() -> Result<u32, String> {
    let file = match File::open("./files/input.txt") {
        Ok(file) => file,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let lines = BufReader::new(file).lines();
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut numbers: (Option<(usize, u32)>, Option<(usize, u32)>);
    let mut sum: u32 = 0;

    for line in lines {
        numbers = (None, None);

        if let Err(err) = line {
            return Err(err.to_string());
        }

        let line = line.unwrap();

        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                numbers.0 = Some((i, c.to_digit(10).unwrap()));
                break;
            }
        }

        for (i, c) in line.chars().rev().enumerate() {
            if c.is_ascii_digit() {
                numbers.1 = Some((line.len() - i - 1, c.to_digit(10).unwrap()));
                break;
            }
        }

        for (i, word) in words.iter().enumerate() {
            for (j, _) in line.match_indices(word).into_iter() {
                let v = (i + 1) as u32;
                match numbers {
                    (Some(first), Some(last)) => {
                        if j < first.0 {
                            numbers.0 = Some((j, v));
                        }
                        if j > last.0 {
                            numbers.1 = Some((j, v));
                        }
                    }
                    (None, None) => {
                        numbers.0 = Some((j, v));
                        numbers.1 = Some((j, v));
                    }
                    (_, _) => {
                        todo!()
                    }
                }
            }
        }

        sum += 10 * numbers.0.unwrap().1 + numbers.1.unwrap().1;
    }
    return Ok(sum);
}

fn main() {
    println!("Part 1: {}", part1().unwrap());
    println!("Part 2: {}", part2().unwrap());
}
