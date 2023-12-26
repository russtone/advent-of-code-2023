use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom},
};

fn main() -> Result<(), Error> {
    let mut file = File::open("files/input.txt")?;

    println!("Part 1: {}", solve(&mut file, 2)?);

    file.seek(SeekFrom::Start(0))?;

    println!("Part 2: {}", solve(&mut file, 1000000)?);

    Ok(())
}

fn solve<R: Read>(buf: &mut R, expansion_coeff: usize) -> Result<usize, Error> {
    let universe = Universe::from_buf(buf, expansion_coeff)?;
    let mut res: usize = 0;
    let n = universe.galaxies.len();

    for i in 0..n {
        for j in i + 1..n {
            let a = universe.galaxies[i];
            let b = universe.galaxies[j];
            res += Universe::distance(a, b);
        }
    }
    Ok(res)
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
    galaxies: Vec<(usize, usize)>,
}

impl Universe {
    fn from_buf<R: Read>(buf: R, expansion_coeff: usize) -> Result<Self, Error> {
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
            *row = *row + (expansion_coeff - 1) * expanded_rows.range(..*row).count();
            *col = *col + (expansion_coeff - 1) * expanded_cols.range(..*col).count();
        }

        Ok(Universe { galaxies })
    }

    fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }
}
