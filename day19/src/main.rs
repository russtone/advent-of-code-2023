use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

type Result<T> = std::result::Result<T, &'static str>;

fn main() -> Result<()> {
    let mut file = File::open("files/input.txt").map_err(|_| "can't open file")?;

    let data = parse(&mut file)?;

    println!("{:?}", solve(&data));

    Ok(())
}

fn solve(data: &Data) -> u64 {
    let mut res = 0;

    'outer: for rating in &data.ratings {
        if let Some(workflow) = data.workflows.get("in") {
            let mut workflow_ref = workflow;

            while let Some(action) = workflow_ref.apply(rating) {
                match action {
                    Action::Accept => {
                        res += rating.values.values().sum::<u64>();
                        continue 'outer;
                    }
                    Action::Reject => continue 'outer,
                    Action::Goto(name) => {
                        if let Some(next_workflow) = data.workflows.get(&name) {
                            workflow_ref = next_workflow;
                        } else {
                            panic!("invalid workflow");
                        }
                    }
                }
            }
        } else {
            panic!("invalid workflow");
        }
    }


    res
}

fn parse<R: Read>(buf: &mut R) -> Result<Data> {
    let lines = BufReader::new(buf).lines();
    let mut workflows = HashMap::new();
    let mut ratings = Vec::new();
    let mut parsing_workflows = true;

    for line in lines {
        let line = line.map_err(|_| "can't get line")?;
        if line.is_empty() {
            parsing_workflows = false;
            continue;
        }

        if parsing_workflows {
            let workflow: Workflow = line.parse()?;
            workflows.insert(workflow.name.clone(), workflow);
        } else {
            ratings.push(line.parse()?);
        }
    }

    Ok(Data { workflows, ratings })
}

#[derive(Debug)]
struct Data {
    workflows: HashMap<String, Workflow>,
    ratings: Vec<Rating>,
}

#[derive(Debug, Clone)]
struct Rating {
    values: HashMap<Category, u64>,
}

impl FromStr for Rating {
    type Err = &'static str;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut values = HashMap::new();
        let parsed: Vec<(Category, u64)> = s
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split(",")
            .map(|p| {
                let mut parts = p.splitn(2, "=");
                let category: Category = parts.next().ok_or("missing category")?.parse()?;
                let value: u64 = parts
                    .next()
                    .ok_or("missing value")?
                    .parse()
                    .map_err(|_| "failed to parse value")?;

                Ok((category, value))
            })
            .collect::<Result<Vec<(Category, u64)>>>()?;

        for (c, v) in &parsed {
            values.insert(*c, *v);
        }

        Ok(Rating { values })
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = &'static str;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '{');
        let name = parts.next().ok_or("missing name")?.to_string();

        let rules_part = parts.next().ok_or("missing rules")?.trim_end_matches('}');

        let rules: Vec<Rule> = rules_part
            .split(',')
            .map(|s| s.parse::<Rule>())
            .collect::<Result<Vec<Rule>>>()?;

        Ok(Workflow { name, rules })
    }
}

impl Workflow {
    fn apply(&self, rating: &Rating) -> Option<Action> {
        for rule in &self.rules {
            match rule {
                Rule::Uncoditional(action) => {
                    return Some(action.clone());
                }
                Rule::Conditional(condition, action) => {
                    if condition.check(&rating.values) {
                        return Some(action.clone());
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug)]
enum Rule {
    Conditional(Condition, Action),
    Uncoditional(Action),
}

impl FromStr for Rule {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((cond, act)) = s.split_once(':') {
            Ok(Rule::Conditional(cond.parse()?, act.parse()?))
        } else {
            Ok(Rule::Uncoditional(s.parse()?))
        }
    }
}

#[derive(Debug)]
struct Condition {
    category: Category,
    sign: Sign,
    value: u64,
}

impl FromStr for Condition {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() < 3 {
            return Err("input is too short");
        }

        let category: Category = s[0..1].parse()?;
        let sign: Sign = s[1..2].parse()?;
        let value: u64 = s[2..].parse().map_err(|_| "failed to parse value")?;

        Ok(Condition {
            category,
            sign,
            value,
        })
    }
}

impl Condition {
    fn check(&self, values: &HashMap<Category, u64>) -> bool {
        let value = values.get(&self.category).unwrap();
        match self.sign {
            Sign::Gt => value > &self.value,
            Sign::Lt => value < &self.value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "a" => Ok(Category::A),
            "s" => Ok(Category::S),
            _ => Err("invalid category"),
        }
    }
}

#[derive(Debug)]
enum Sign {
    Gt,
    Lt,
}

impl FromStr for Sign {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "<" => Ok(Sign::Lt),
            ">" => Ok(Sign::Gt),
            _ => Err("invalid sign"),
        }
    }
}

#[derive(Debug, Clone)]
enum Action {
    Accept,
    Reject,
    Goto(String),
}

impl FromStr for Action {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Action::Accept),
            "R" => Ok(Action::Reject),
            name => Ok(Action::Goto(name.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_rule() {}

    #[test]
    fn test_parse_workflow() {}
}
