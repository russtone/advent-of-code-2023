use std::{error, fs::File, io::Read};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let mut file = File::open("files/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let s = contents.trim_end();

    println!("{}", hash_seq(s));

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
