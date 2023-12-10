use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Dir {
    Right,
    Left,
    Up,
    Down
}

use Dir::*;

impl Dir {
    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Right => (1, 0),
            Left => (-1, 0),
            Up => (0, -1),
            Down => (0, 1)
        }
    }

    fn displace(&self, pos: &(i32, i32)) -> (i32, i32) {
        let d = self.to_tuple();
        (pos.0 + d.0, pos.1 + d.1)
    }

    fn next(&self, c: u8) -> Option<Dir> {
        match (self, c) {
            (Right, b'-') => Some(Right),
            (Right, b'J') => Some(Up),
            (Right, b'7') => Some(Down),
            (Left, b'-') => Some(Left),
            (Left, b'F') => Some(Down),
            (Left, b'L') => Some(Up),
            (Up, b'|') => Some(Up),
            (Up, b'F') => Some(Right),
            (Up, b'7') => Some(Left),
            (Down, b'|') => Some(Down),
            (Down, b'L') => Some(Right),
            (Down, b'J') => Some(Left),
            _ => None
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
    let mut field = vec![];
    let mut start = (0, 0);
    for (y, line) in io::BufReader::new(file).lines().map(Result::unwrap).enumerate() {
        if line.contains('S') {
            start.0 = line.find('S').unwrap() as i32;
            start.1 = y as i32;
        }
        field.push(line.into_bytes());
    }
    let height = field.len() as i32;
    let width = field[0].len() as i32;
    //println!("{start:?}");

    let mut crossings = vec![];
    for _ in 0..field.len() {
        crossings.push(vec![]);
    }
    for dir in vec![Right, Left, Up, Down] {
        let mut pos = dir.displace(&start);
        if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
            continue;
        }
        if dir.next(field[pos.1 as usize][pos.0 as usize]).is_none() {
            continue;
        }
        if dir == Down || dir == Up {
            crossings[start.1 as usize].push((start.0, dir.to_tuple().1));
        }
        //println!("{dir:?}");
        let mut n = 1;
        let mut d = dir;
        //println!("{pos:?}");
        while let Some(d_new) = d.next(field[pos.1 as usize][pos.0 as usize]) {
            if d == Down || d == Up {
                crossings[pos.1 as usize].push((pos.0, d.to_tuple().1));
            }
            if d_new == Down || d_new == Up {
                crossings[pos.1 as usize].push((pos.0, d_new.to_tuple().1));
            }
            d = d_new;
            pos = d.displace(&pos);
            n += 1;
            //println!("{pos:?} {}", field[pos.1 as usize][pos.0 as usize] as char);
        }
        //println!("{pos:?} {} {}", field[pos.1 as usize][pos.0 as usize], n);
        if d == Down || d == Up {
            crossings[pos.1 as usize].push((pos.0, d.to_tuple().1));
        }
        println!("{} {}", n, n / 2);
        break;
    }

    let mut volume = 0;
    for mut line in crossings {
        line.sort();
        let mut last = 0;
        let mut wind = 0;
        for (cross, sign) in line {
            match (wind, sign) {
                (1, 1) | (-1, -1) => last = cross,
                (2, -1) => volume += cross - last - 1,
                (-2, 1) => volume -= cross - last - 1,
                _ => ()
            }
            wind += sign;
            //print!("{}: {} ", cross, volume);
        }
    }
    println!("{}", volume.abs());
}
