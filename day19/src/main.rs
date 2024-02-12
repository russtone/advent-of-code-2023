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

    println!("{:?}", part1::solve(&data));
    println!("{:?}", part2::solve(&data));

    Ok(())
}

mod part2 {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    struct State {
        x: (u64, u64),
        m: (u64, u64),
        a: (u64, u64),
        s: (u64, u64),
    }

    impl State {
        fn apply_condition(&self, condition: &Condition) -> Option<State> {
            let x = self.x;
            let m = self.m;
            let a = self.a;
            let s = self.s;

            match condition.category {
                Category::X => Some(State {
                    x: apply_condition(condition, x)?,
                    m,
                    a,
                    s,
                }),
                Category::M => Some(State {
                    x,
                    m: apply_condition(condition, m)?,
                    a,
                    s,
                }),
                Category::A => Some(State {
                    x,
                    m,
                    a: apply_condition(condition, a)?,
                    s,
                }),
                Category::S => Some(State {
                    x,
                    m,
                    a,
                    s: apply_condition(condition, s)?,
                }),
            }
        }

        fn count(&self) -> u64 {
            (self.x.1 - self.x.0 + 1)
                * (self.m.1 - self.m.0 + 1)
                * (self.a.1 - self.a.0 + 1)
                * (self.s.1 - self.s.0 + 1)
        }
    }

    fn apply_condition(condition: &Condition, range: (u64, u64)) -> Option<(u64, u64)> {
        match condition.sign {
            Sign::Gt => {
                if condition.value >= range.1 {
                    None
                } else if condition.value > range.0 && condition.value < range.1 {
                    Some((condition.value + 1, range.1))
                } else {
                    Some(range)
                }
            }
            Sign::Lt => {
                if condition.value <= range.0 {
                    None
                } else if condition.value > range.0 && condition.value < range.1 {
                    Some((range.0, condition.value - 1))
                } else {
                    Some(range)
                }
            }
        }
    }

    fn neg(condition: &Condition) -> Condition {
        Condition {
            category: condition.category,
            sign: match condition.sign {
                Sign::Gt => Sign::Lt,
                Sign::Lt => Sign::Gt,
            },
            value: match condition.sign {
                Sign::Gt => condition.value + 1,
                Sign::Lt => condition.value - 1,
            },
        }
    }

    pub fn solve(data: &Data) -> u64 {
        let mut res = 0;
        let start = State {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        };

        let states = process(start, "in", 0, data);


        for state in &states {
            res += state.count();
        }

        res
    }

    fn process(state: State, workflow_name: &str, rule_index: usize, data: &Data) -> Vec<State> {
        let mut res = Vec::new();

        if let Some(workflow) = data.workflows.get(workflow_name) {
            let mut current = state;
            for ri in rule_index..workflow.rules.len() {
                let rule = &workflow.rules[ri];
                match rule {
                    Rule::Unconditional(action) => match action {
                        Action::Accept => res.push(current.clone()),
                        Action::Reject => {}
                        Action::Goto(name) => res.extend(process(current, name, 0, data)),
                    },
                    Rule::Conditional(condition, action) => {
                        if let Some(s) = current.apply_condition(condition) {
                            match action {
                                Action::Accept => res.push(s.clone()),
                                Action::Reject => {}
                                Action::Goto(name) => res.extend(process(s, name, 0, data)),
                            }
                        }
                        if let Some(next) = current.apply_condition(&neg(condition)) {
                            current = next;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        res
    }
}

mod part1 {
    use super::*;

    pub fn solve(data: &Data) -> u64 {
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
                Rule::Unconditional(action) => {
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
    Unconditional(Action),
}

impl FromStr for Rule {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((cond, act)) = s.split_once(':') {
            Ok(Rule::Conditional(cond.parse()?, act.parse()?))
        } else {
            Ok(Rule::Unconditional(s.parse()?))
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone, Copy)]
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
