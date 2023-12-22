#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

use Dir::*;

impl Dir {
    fn to_int(self) -> i8 {
        match self {
            Up => 0,
            Down => 1,
            Left => 2,
            Right => 3
        }
    }

    fn to_tuple(self) -> (i32, i32) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0)
        }
    }

    fn after(&self, ch: u8) -> (Dir, Option<Dir>) {
        match (self, ch) {
            (_, b'.') | (Up | Down, b'|') | (Left | Right, b'-') => (*self, None),
            (Left, b'/') | (Right, b'\\') => (Down, None),
            (Left, b'\\') | (Right, b'/') => (Up, None),
            (Up, b'/') | (Down, b'\\') => (Right, None),
            (Down, b'/') | (Up, b'\\') => (Left, None),
            (Left | Right, b'|') => (Up, Some(Down)),
            (Up | Down, b'-') => (Left, Some(Right)),
            _ => panic!("{ch}")
        }
    }
}

#[derive(Clone, Default)]
struct Seen(i8);

impl Seen {
    fn set(&mut self, d: Dir) {
        self.0 |= 1 << d.to_int();
    }

    fn get(&self, d: Dir) -> bool {
        self.0 & (1 << d.to_int()) > 0
    }

    fn any(&self) -> bool {
        self.0 > 0
    }
}



trait Pos {
    fn displace(&self, dir: Dir) -> Self;
    fn within(&self, size: (i32, i32)) -> bool;
}

impl Pos for (i32, i32) {
    fn displace(&self, dir: Dir) -> (i32, i32) {
        let t = dir.to_tuple();
        (self.0 + t.0, self.1 + t.1)
    }

    fn within(&self, (w, h): (i32, i32)) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < w && self.1 < h
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let field = std::fs::read_to_string(filename).unwrap().lines()
        .map(|line| line.as_bytes().to_owned())
        .collect::<Vec<_>>();
    let (w, h) = (field[0].len() as i32, field.len() as i32);

    // Part A
    println!("{}", score(&field, (0,0), Right));

    // Part B
    let mut max = 0;
    for x in 0..w {
        max = std::cmp::max(max, score(&field, (x, 0), Down));
        max = std::cmp::max(max, score(&field, (x, h - 1), Up));
    }

    for y in 0..h {
        max = std::cmp::max(max, score(&field, (0, y), Right));
        max = std::cmp::max(max, score(&field, (w - 1, y), Left));
    }

    println!("{max}");
}

fn score(field: &Vec<Vec<u8>>, ini_pos: (i32, i32), ini_dir: Dir) -> u32 {
    let (w, h) = (field[0].len() as i32, field.len() as i32);
    let mut hist = vec![];
    for _ in 0..h {
        let mut row = vec![];
        row.resize(w as usize, Seen::default());
        hist.push(row);
    }

    let mut stack = vec![];
    stack.push((ini_pos, ini_dir));
    while let Some((pos, dir)) = stack.pop() {
        if !pos.within((w, h)) {
            continue;
        }
        let ch = field[pos.1 as usize][pos.0 as usize];
        let seen = &mut hist[pos.1 as usize][pos.0 as usize];
        if seen.get(dir) {
            continue;
        }
        seen.set(dir);
        let (d1, d2) = dir.after(ch);
        stack.push((pos.displace(d1), d1));
        if let Some(d2) = d2 {
            stack.push((pos.displace(d2), d2));
        }
    }

    hist.iter().map(|row| row.iter().map(|seen| seen.any() as u32).sum::<u32>()).sum::<u32>()
}
