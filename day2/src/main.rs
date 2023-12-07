use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Game {
    index: u32,
    cubes: Vec<Cubes>,
}

impl Game {
    fn parse(s: &str) -> Game {
        let (game_info, rounds_str) = s.split_once(": ").unwrap();
        let (_, game_index_str) = game_info.split_once(" ").unwrap();

        return Game {
            index: game_index_str.parse().unwrap(),
            cubes: rounds_str.split("; ").map(Cubes::parse).collect(),
        };
    }

    fn min_set(&self) -> Cubes {
        let mut cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };

        for c in &self.cubes {
            cubes.red = max(cubes.red, c.red);
            cubes.green = max(cubes.green, c.green);
            cubes.blue = max(cubes.blue, c.blue);
        }

        return cubes;
    }
}

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn parse(s: &str) -> Cubes {
        let mut cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        for part in s.split(", ").into_iter() {
            let (count_str, color) = part.split_once(" ").unwrap();
            let count = count_str.parse().unwrap();
            match color {
                "red" => cubes.red = count,
                "green" => cubes.green = count,
                "blue" => cubes.blue = count,
                _ => panic!("unexpected color {}", color),
            }
        }
        return cubes;
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn main() {
    let file = match File::open("./files/input.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    };

    let lines = BufReader::new(file).lines();

    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        match line {
            Ok(line) => games.push(Game::parse(&line)),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1)
            }
        }
    }

    let part1_answer: u32 = games
        .iter()
        .filter(|g| {
            let bound = g.min_set();
            bound.red <= 12 && bound.green <= 13 && bound.blue <= 14
        })
        .map(|g| g.index)
        .sum();

    println!("Part 1: {:?}", part1_answer);

    let part2_answer: u32 = games.iter().map(|g| g.min_set().power()).sum();

    println!("Part 1: {:?}", part2_answer);
}
