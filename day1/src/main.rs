use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = match File::open("./files/input.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error: {:#?}", err);
            std::process::exit(1);
        }
    };

    let lines = BufReader::new(file).lines();

    let mut numbers: (Option<u32>, Option<u32>);
    let mut sum: u32 = 0;

    for line in lines {
        numbers = (None, None);

        if let Err(err) = line {
            eprintln!("Error: {:#?}", err);
            std::process::exit(1);
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
    println!("Answer: {}", sum)
}
