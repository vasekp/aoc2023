use std::fs::File;
use std::io::{self, BufRead};
use std::collections::hash_map::HashMap;

mod field {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[derive(PartialEq)]
    pub enum Orientation {
        North,
        East,
        West,
        South
    }

    use Orientation::*;

    pub struct Field {
        field: Vec<Vec<u8>>,
        len: usize,
        ori: Orientation
    }

    impl Field {
        pub fn from(field: Vec<Vec<u8>>) -> Field {
            assert!(field.len() == field[0].len());
            let len = field.len();
            Field { field, len, ori: West }
        }

        fn trans(&mut self) {
            for i in 0..self.len {
                for j in 0..self.len {
                    if i < j {
                        // Can't use std::mem::swap :-(
                        let tmp = self.field[i][j];
                        self.field[i][j] = self.field[j][i];
                        self.field[j][i] = tmp;
                    }
                }
            }
        }

        fn rev(&mut self) {
            self.field.reverse();
        }

        fn rot_ccw(&mut self) {
            self.trans();
            self.rev();
            self.ori = match self.ori {
                North => East,
                East => South,
                South => West,
                West => North
            }
        }

        fn rot_cw(&mut self) {
            self.rev();
            self.trans();
            self.ori = match self.ori {
                East => North,
                South => East,
                West => South,
                North => West
            }
        }

        fn fall_left(&mut self) {
            for row in &mut self.field {
                let mut next_pos = 0;
                for ix in 0..self.len {
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

        fn orient(&mut self, ori: Orientation) {
            match (&self.ori, &ori) {
                (x, y) if x == y => (),
                (North, East) | (East, South) | (South, West) | (West, North) => self.rot_ccw(),
                (East, North) | (South, East) | (West, South) | (North, West) => self.rot_cw(),
                _ => { self.rot_cw(); self.rot_cw() }
            }
            assert!(self.ori == ori);
        }

        pub fn fall(&mut self, ori: Orientation) {
            self.orient(ori);
            self.fall_left();
        }

        pub fn hash(&mut self) -> u64 {
            let mut hasher = DefaultHasher::new();
            self.field.hash(&mut hasher);
            hasher.finish()
        }

        pub fn score(&mut self) -> usize {
            self.orient(North);
            let mut total = 0usize;
            for row in &self.field {
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
            for row in &self.field {
                writeln!(f, "{}", std::str::from_utf8(row).unwrap())?
            }
            Ok(())
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines().map(Result::unwrap)
        .map(|line| line.as_bytes().to_owned())
        .collect();
    let mut field = field::Field::from(lines);

    use field::Orientation::*;
    field.fall(North);
    // Part A
    println!("{}", field.score());

    // Part B
    let mut map = HashMap::new();
    let mut period = None;
    let target = 1_000_000_000;
    for i in 1..=target {
        field.fall(North); // no-op right after Part A
        field.fall(West);
        field.fall(South);
        field.fall(East);
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
