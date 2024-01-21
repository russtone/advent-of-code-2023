use std::{collections::HashMap, error, fs::File, io::Read};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let mut file = File::open("files/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let s = contents.trim_end();

    println!("Part 1: {}", hash_seq(s));

    let mut boxes: HashMap<u64, Vec<(String, u64)>> = HashMap::new();

    s.split(",").for_each(|cmd| {
        if cmd.ends_with("-") {
            let label = cmd.strip_suffix("-").unwrap();
            boxes
                .entry(hash(&label))
                .and_modify(|e| e.retain(|(l, _)| l != label));
        } else {
            let (label, flen) = cmd.split_once("=").unwrap();
            let value = (label.to_string(), flen.parse().unwrap());

            boxes
                .entry(hash(&label))
                .and_modify(|e| {
                    if let Some(element) = e.iter_mut().find(|x| x.0 == label) {
                        *element = value.to_owned();
                    } else {
                        e.push(value.to_owned())
                    }
                })
                .or_insert(vec![value]);
        }
    });

    let mut res: u64 = 0;

    for (bi, lenses) in &boxes {
        for (i, (_, flen)) in lenses.iter().enumerate() {
            res += (bi + 1) * (i as u64 + 1) * flen;
        }
    }

    println!("Part 2: {}", res);

    Ok(())
}

fn hash_seq(s: &str) -> u64 {
    s.split(",").map(|chunk| hash(chunk)).sum()
}

fn hash(s: &str) -> u64 {
    let mut state: u64 = 0;

    for c in s.chars() {
        state += (c as u8) as u64;
        state *= 17;
        state %= 256;
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash(&"HASH"), 52);
        assert_eq!(hash(&"rn=1"), 30);
        assert_eq!(hash(&"pc=6"), 214);
        assert_eq!(hash(&"qp-"), 14);
    }

    #[test]
    fn test_hash_seq() {
        assert_eq!(
            hash_seq(&"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }
}
