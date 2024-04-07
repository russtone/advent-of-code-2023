use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{DMatrix, RealField};

type Result<T> = std::result::Result<T, &'static str>;

fn main() -> Result<()> {
    let file = File::open("files/input.txt").map_err(|_| "can't open file")?;
    let lines = BufReader::new(file).lines();

    let mut edges: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut nodes: BTreeSet<String> = BTreeSet::new();

    for line in lines {
        let line = line.map_err(|_| "can't read line")?;
        let (node, adj) = line.split_once(": ").ok_or("fail to split")?;
        nodes.insert(node.to_owned());

        for n in adj.split(" ") {
            nodes.insert(n.to_owned());
            edges
                .entry(node.to_owned())
                .and_modify(|s| {
                    if n != node {
                        s.insert(n.to_owned());
                    }
                })
                .or_insert(if n != node {
                    BTreeSet::from_iter(vec![n.to_owned()])
                } else {
                    BTreeSet::new()
                });
            edges
                .entry(n.to_owned())
                .and_modify(|s| {
                    if n != node {
                        s.insert(node.to_owned());
                    }
                })
                .or_insert(if n != node {
                    BTreeSet::from_iter(vec![node.to_owned()])
                } else {
                    BTreeSet::new()
                });
        }
    }

    // https://patterns.eecs.berkeley.edu/?page_id=571#4_Spectral_Bisection
    // https://github.com/alexcani/adventofcode2023/blob/master/src/bin/25.rs#L62
    let mut matrix: DMatrix<f32> = DMatrix::<f32>::zeros(nodes.len(), nodes.len());

    for (i, n) in nodes.iter().enumerate() {
        matrix[(i, i)] = edges.get(n).unwrap().len() as f32;
        for e in edges.get(n).unwrap() {
            let j = nodes.iter().enumerate().find(|&(_, it)| it == e).unwrap().0;
            matrix[(i, j)] = -1.0
        }
    }

    let eigen = matrix.clone().symmetric_eigen();

    let mut min_value = f32::MAX;
    let mut min_index = 0;
    let mut second_min_value = f32::MAX;
    let mut second_min_index = 0;

    for (i, &v) in eigen.eigenvalues.iter().enumerate() {
        if v < min_value {
            second_min_value = min_value;
            second_min_index = min_index;
            min_value = v;
            min_index = i;
        } else if v < second_min_value {
            second_min_value = v;
            second_min_index = i;
        }
    }

    let vector = eigen.eigenvectors.column(second_min_index);

    let mut pos = 0;
    let mut neg = 0;

    for v in vector.iter() {
        if v.is_sign_positive() {
            pos += 1;
        } else {
            neg += 1;
        }
    }

    println!("{:?}", pos * neg);

    Ok(())
}

fn graphviz(nodes: &HashSet<String>, edges: &HashMap<String, HashSet<String>>) {
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut res = String::new();

    res.push_str("graph Components {\n");

    for n in nodes {
        res.push_str(&format!("\"{}\";\n", n));
    }

    for n in edges.keys() {
        for e in edges.get(n).unwrap() {
            if !seen.contains(&(n.to_string(), e.to_string()))
                && !seen.contains(&(e.to_string(), n.to_string()))
            {
                seen.insert((n.to_string(), e.to_string()));
                res.push_str(&format!("\"{}\" -- \"{}\";\n", n, e));
            }
        }
    }

    res.push_str("}\n");

    println!("{}", res);
}
