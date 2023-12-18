use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut commands: Vec<Command> = Vec::new();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for (i, line) in lines.enumerate() {
        let line = line?;

        if i == 0 {
            commands = parse_commands(&line)?;
        } else if i > 1 {
            let (key, value) = parse_line(&line)?;
            map.insert(key, value);
        }
    }

    if commands.len() == 0 {
        return Err(Error::EmptyCommands);
    }

    let mut key: String = "AAA".to_owned();
    let mut cmd_index: usize = 0;
    let mut steps: u32 = 0;

    while key != "ZZZ" {
        let cmd = &commands[cmd_index % commands.len()];
        let values = map.get(&key).unwrap();
        match cmd {
            Command::Left => key = values.0.to_owned(),
            Command::Right => key = values.1.to_owned(),
        }
        cmd_index += 1;
        steps += 1;
    }

    println!("{}", steps);

    Ok(())
}

#[derive(Debug)]
struct ParseError;

fn parse_line(s: &str) -> Result<(String, (String, String)), Error> {
    let (key, rest) = s.split_once(" = ").ok_or_else(|| Error::ParseError)?;

    let ss: &str = &rest[1..rest.len() - 1];
    let (left, right) = ss.split_once(", ").ok_or_else(|| Error::ParseError)?;

    return Ok((key.to_owned(), (left.to_owned(), right.to_owned())));
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    ParseError,
    EmptyCommands,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

#[derive(Debug)]
enum Command {
    Left,
    Right,
}

fn parse_commands(s: &str) -> Result<Vec<Command>, Error> {
    let mut commands = Vec::new();
    for c in s.chars() {
        match c {
            'R' => commands.push(Command::Right),
            'L' => commands.push(Command::Left),
            _ => return Err(Error::ParseError),
        }
    }

    Ok(commands)
}
