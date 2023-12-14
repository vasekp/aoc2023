use std::fs::File;
use std::io::{self, BufRead};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::{HashMap, DefaultHasher};

struct Field(Vec<Vec<u8>>);

impl Field {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn trans(&mut self) {
        for i in 0..self.len() {
            for j in 0..self.len() {
                if i < j {
                    // Can't use std::mem::swap :-(
                    let tmp = self.0[i][j];
                    self.0[i][j] = self.0[j][i];
                    self.0[j][i] = tmp;
                }
            }
        }
    }

    fn fall(&mut self) {
        for row in &mut self.0 {
            let mut next_pos = 0;
            for ix in 0..row.len() {
                if row[ix] == b'O' {
                    if ix != next_pos {
                        row[next_pos] = b'O';
                        row[ix] = b'.';
                    }
                    next_pos += 1;
                } else if row[ix] == b'#' {
                    next_pos = ix + 1;
                }
            }
        }
    }

    fn reverse(&mut self) {
        self.0.reverse();
    }

    fn hash(&mut self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn score(&self) -> usize {
        let mut total = 0usize;
        for row in &self.0 {
            for (ix, ch) in row.iter().rev().enumerate() {
                if ch == &b'O' {
                    total += ix + 1;
                }
            }
        }
        total
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.0 {
            writeln!(f, "{}", std::str::from_utf8(row).unwrap())?
        }
        Ok(())
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut field = vec![];
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        field.push(line.as_bytes().to_owned());
    }
    assert!(field.len() == field[0].len());
    let mut field = Field(field);
    let mut map = HashMap::new();
    let mut period = None;
    let target = 1_000_000_000;

    // ← west ↑ north
    field.trans(); // ← north ↑ west
    field.fall();
    // Part A
    println!("{}", field.score());

    // Part B
    for i in 1..=target {
        field.fall(); // fall NORTH
        field.trans(); // ← west ↑ north
        field.fall(); // fall WEST
        field.reverse(); // ← west ↑ south
        field.trans(); // ← south ↑ west
        field.fall(); // fall SOUTH
        field.reverse(); // ← south ↑ east
        field.trans(); // ← east ↑ south
        field.fall(); // fall EAST
        field.reverse(); // ← east ↑ north
        field.trans(); // ← north ↑ east
        field.reverse(); // ← north ↑ west
        //println!("{i}: {}", field.score());
        let hash = field.hash();
        match period {
            None => {
                if let Some(j) = map.get(&hash) {
                    period = Some(i - j);
                    //println!("Period: {}", i - j);
                } else {
                    map.insert(hash, i);
                }
            },
            Some(p) => {
                if (target - i) % p == 0 {
                    break;
                }
            }
        }
    }
    println!("{}", field.score());
}

