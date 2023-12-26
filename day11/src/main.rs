use std::{
    collections::{BTreeSet, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

fn main() -> Result<(), Error> {
    let file = File::open("files/input.txt")?;

    let universe = Universe::from_buf(file)?;
    let mut res: usize = 0;
    let n = universe.galaxies.len();

    for i in 0..n {
        for j in i + 1..n {
            let a = universe.galaxies[i];
            let b = universe.galaxies[j];
            res += Universe::distance(a, b);
            // println!("{:?} {:?} {}", a, b, Universe::distance(a, b))
        }
    }

    println!("{}", res);

    Ok(())
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

#[derive(Debug)]
struct Universe {
    rows: usize,
    cols: usize,
    galaxies: Vec<(usize, usize)>,
}

impl Universe {
    fn from_buf<R: Read>(buf: R) -> Result<Self, Error> {
        let lines = BufReader::new(buf).lines();
        let mut rows: usize = 0;
        let mut cols: usize = 0;
        let mut galaxies = Vec::new();

        for (row, line) in lines.enumerate() {
            let line = line?;
            if row == 0 {
                cols = line.len();
            }

            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => galaxies.push((row, col)),
                    _ => {}
                }
            }
            rows += 1;
        }

        let mut expanded_rows: BTreeSet<_> = (0..rows - 1).collect();
        let mut expanded_cols: BTreeSet<_> = (0..cols - 1).collect();

        for (row, col) in &galaxies {
            expanded_rows.remove(row);
            expanded_cols.remove(col);
        }

        for (row, col) in &mut galaxies {
            *row = *row + expanded_rows.range(..*row).count();
            *col = *col + expanded_cols.range(..*col).count();
        }

        Ok(Universe {
            rows,
            cols,
            galaxies,
        })
    }

    fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }
}
