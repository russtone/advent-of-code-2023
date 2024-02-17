use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader, Read},
};

type Result<T> = std::result::Result<T, &'static str>;

fn main() -> Result<()> {
    let mut file = File::open("files/input.txt").map_err(|_| "can't open file")?;
    let mut machine = parse(&mut file)?;

    println!("Part1: {}", part1::solve(&mut machine)?);

    machine.reset();

    println!("Part2: {:?}", part2::solve(&mut machine)?);

    Ok(())
}

mod part2 {
    use super::*;

    pub fn solve(machine: &mut Machine) -> Result<u64> {
        let outputs: Vec<&str> = vec!["jz", "ft", "sv", "ng"];
        let mut counts: Vec<u64> = Vec::new();

        for output in outputs {
            counts.push(part2::count(machine, |from, to, pulse| {
                from == output && to == "xm" && pulse == Pulse::High
            })?);
            machine.reset();
        }

        let mut res = counts[0];

        for i in 1..counts.len() {
            res = lcm(res, counts[i]);
        }

        Ok(res)
    }

    fn count<F>(machine: &mut Machine, break_when: F) -> Result<u64>
    where
        F: Fn(&str, &str, Pulse) -> bool,
    {
        let mut signals: VecDeque<(String, String, Pulse)> = VecDeque::new();

        let mut c: u64 = 0;

        'outer: loop {
            signals.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
            c += 1;

            while let Some((from, to, pulse)) = signals.pop_front() {
                if break_when(&from, &to, pulse) {
                    break 'outer;
                }
                machine.modules.entry(to).and_modify(|m| {
                    m.send((&from, pulse)).iter().for_each(|out| {
                        signals.push_back((m.name().to_string(), out.0.to_string(), out.1))
                    });
                });
            }
        }
        Ok(c)
    }

    pub fn lcm(a: u64, b: u64) -> u64 {
        (a * b) / gcd(a, b)
    }

    pub fn gcd(a: u64, b: u64) -> u64 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            if b < a {
                std::mem::swap(&mut b, &mut a);
            }
            b %= a;
        }
        a
    }
}

mod part1 {
    use super::*;

    pub fn solve(machine: &mut Machine) -> Result<u64> {
        let mut signals: VecDeque<(String, String, Pulse)> = VecDeque::new();

        let mut low: u64 = 0;
        let mut high: u64 = 0;

        for _ in 0..1000 {
            signals.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

            while let Some((from, to, pulse)) = signals.pop_front() {
                match pulse {
                    Pulse::Low => low += 1,
                    Pulse::High => high += 1,
                }
                machine.modules.entry(to).and_modify(|m| {
                    m.send((&from, pulse)).iter().for_each(|out| {
                        signals.push_back((m.name().to_string(), out.0.to_string(), out.1))
                    });
                });
            }
        }
        Ok(low * high)
    }
}

fn parse<R: Read>(buf: &mut R) -> Result<Machine> {
    let lines = BufReader::new(buf).lines();
    let mut modules = HashMap::new();
    let mut io: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let line = line.map_err(|_| "can't get line")?;

        if let Some((left, right)) = line.split_once(" -> ") {
            let outputs = right.split(", ").collect::<Vec<&str>>();

            let module: Box<dyn Module> = if left == "broadcaster" {
                Box::new(Broadcaster::new(outputs.clone()))
            } else {
                match left.chars().next() {
                    Some(c) => match c {
                        '%' => Box::new(FlipFlop::new(&left[1..], outputs.clone())),
                        '&' => Box::new(Conjunction::new(&left[1..], outputs.clone())),
                        _ => return Err("invalid module"),
                    },
                    None => return Err("invalid module"),
                }
            };

            for output in outputs {
                io.entry(output.to_string())
                    .and_modify(|e| e.push(module.name().to_string()))
                    .or_insert(vec![module.name().to_string()]);
            }

            modules.insert(module.name().to_string(), module);
        } else {
            return Err("fail to split line");
        }
    }

    for (output, inputs) in &io {
        modules
            .entry(output.to_string())
            .and_modify(|e| inputs.iter().for_each(|input| e.add_input(input)));
    }

    Ok(Machine { modules })
}

#[derive(Debug)]
struct Machine {
    modules: HashMap<String, Box<dyn Module>>,
}

impl Machine {
    fn graphviz(&self) -> String {
        let mut res = String::new();

        res.push_str("digraph Machine {\n");

        for m in self.modules.values() {
            res.push_str(&m.graphviz_node());
        }

        for m in self.modules.values() {
            res.push_str(&m.graphviz_edges());
        }

        res.push_str("}\n");

        res
    }

    fn reset(&mut self) {
        for m in self.modules.values_mut() {
            m.reset();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Low => write!(f, "low"),
            Pulse::High => write!(f, "high"),
        }
    }
}

trait Module: Debug {
    fn name(&self) -> &str;
    fn add_input(&mut self, name: &str);
    fn send(&mut self, signal: (&str, Pulse)) -> Vec<(String, Pulse)>;
    fn reset(&mut self);
    fn graphviz_node(&self) -> String;
    fn graphviz_edges(&self) -> String;
}

#[derive(Debug)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl Broadcaster {
    fn new(outputs: Vec<&str>) -> Self {
        Self {
            outputs: outputs.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Module for Broadcaster {
    fn name(&self) -> &str {
        "broadcaster"
    }

    fn add_input(&mut self, _name: &str) {}

    fn send(&mut self, (_, pulse): (&str, Pulse)) -> Vec<(String, Pulse)> {
        self.outputs
            .iter()
            .map(|c| (c.to_string(), pulse))
            .collect::<Vec<(String, Pulse)>>()
    }

    fn reset(&mut self) {}

    fn graphviz_node(&self) -> String {
        format!("\"{}\" [shape=circle];\n", self.name())
    }

    fn graphviz_edges(&self) -> String {
        let mut s = String::new();
        for output in &self.outputs {
            s.push_str(&format!("\"{}\" -> \"{}\";\n", self.name(), output));
        }
        s
    }
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    on: bool,
    outputs: Vec<String>,
}

impl FlipFlop {
    fn new(name: &str, outputs: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            on: false,
            outputs: outputs.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Module for FlipFlop {
    fn name(&self) -> &str {
        &self.name
    }

    fn add_input(&mut self, _name: &str) {}

    fn send(&mut self, (_, pulse): (&str, Pulse)) -> Vec<(String, Pulse)> {
        if pulse == Pulse::Low {
            self.on = !self.on;

            return self
                .outputs
                .iter()
                .map(|c| {
                    (
                        c.to_string(),
                        if self.on { Pulse::High } else { Pulse::Low },
                    )
                })
                .collect::<Vec<(String, Pulse)>>();
        }

        vec![]
    }

    fn reset(&mut self) {
        self.on = false;
    }

    fn graphviz_node(&self) -> String {
        format!(
            "\"{}\" [shape=box,style=filled,fillcolor={}];\n",
            self.name(),
            if self.on { "green" } else { "white" }
        )
    }

    fn graphviz_edges(&self) -> String {
        let mut s = String::new();
        for output in &self.outputs {
            s.push_str(&format!("\"{}\" -> \"{}\";\n", self.name(), output));
        }
        s
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    inputs: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

impl Conjunction {
    fn new(name: &str, outputs: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            inputs: HashMap::new(),
            outputs: outputs.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Module for Conjunction {
    fn name(&self) -> &str {
        &self.name
    }

    fn add_input(&mut self, name: &str) {
        self.inputs.insert(name.to_string(), Pulse::Low);
    }

    fn send(&mut self, (from, pulse): (&str, Pulse)) -> Vec<(String, Pulse)> {
        self.inputs.insert(from.to_string(), pulse);

        let all_high = self.inputs.values().all(|p| p == &Pulse::High);

        self.outputs
            .iter()
            .map(|c| {
                (
                    c.to_string(),
                    if all_high { Pulse::Low } else { Pulse::High },
                )
            })
            .collect::<Vec<(String, Pulse)>>()
    }

    fn reset(&mut self) {
        self.inputs
            .values_mut()
            .for_each(|v: &mut Pulse| *v = Pulse::Low)
    }

    fn graphviz_node(&self) -> String {
        format!("\"{}\" [shape=diamond];\n", self.name())
    }

    fn graphviz_edges(&self) -> String {
        let mut s = String::new();
        for output in &self.outputs {
            s.push_str(&format!("\"{}\" -> \"{}\";\n", self.name(), output));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broadcaster() {
        let mut m = Broadcaster::new(vec![&"a", &"b"]);

        assert_eq!(
            m.send(("", Pulse::High)),
            vec![
                ("a".to_string(), Pulse::High),
                ("b".to_string(), Pulse::High)
            ]
        );
    }

    #[test]
    fn test_flipflop() {
        let mut m = FlipFlop::new("flipflop", vec![&"a", &"b"]);

        assert_eq!(m.send(("", Pulse::High)), vec![]);

        assert_eq!(
            m.send(("", Pulse::Low)),
            vec![
                ("a".to_string(), Pulse::High),
                ("b".to_string(), Pulse::High)
            ]
        );

        assert_eq!(m.send(("", Pulse::High)), vec![]);

        assert_eq!(
            m.send(("", Pulse::Low)),
            vec![("a".to_string(), Pulse::Low), ("b".to_string(), Pulse::Low)]
        );

        assert_eq!(m.send(("", Pulse::High)), vec![]);
    }

    #[test]
    fn test_conjunction() {
        let mut m = Conjunction::new("conjunction", vec![&"a", &"b"]);

        m.add_input("x");
        m.add_input("y");

        assert_eq!(
            m.send(("x", Pulse::High)),
            vec![
                ("a".to_string(), Pulse::High),
                ("b".to_string(), Pulse::High)
            ]
        );

        assert_eq!(
            m.send(("y", Pulse::High)),
            vec![("a".to_string(), Pulse::Low), ("b".to_string(), Pulse::Low)]
        );

        assert_eq!(
            m.send(("x", Pulse::Low)),
            vec![
                ("a".to_string(), Pulse::High),
                ("b".to_string(), Pulse::High)
            ]
        );
    }
}
