use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

use Dir::*;

impl Dir {
    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0)
        }
    }

    fn displace(&self, pos: (i32, i32)) -> (i32, i32) {
        let t = self.to_tuple();
        (pos.0 + t.0, pos.1 + t.1)
    }

    fn cw(&self) -> Dir {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }

    fn ccw(&self) -> Dir {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up
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
    let field: Vec<Vec<u8>> = io::BufReader::new(file).lines().map(Result::unwrap)
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
        .collect();

    // Part A
    println!("{}", min_cost(&field, 1, 3));

    // Part B
    println!("{}", min_cost(&field, 4, 10));
}

fn min_cost(field: &Vec<Vec<u8>>, min_run: u8, max_run: u8) -> u32 {
    let (h, w) = (field.len() as i32, field[0].len() as i32);

    let mut map = HashMap::new();
    let mut stack = VecDeque::new();

    struct StackEntry {
        pos: (i32, i32),
        cost: u32,
        dir: Dir,
        run: u8
    }
    stack.push_back(StackEntry{pos: (1, 0), cost: 0, dir: Right, run: 1});
    stack.push_back(StackEntry{pos: (0, 1), cost: 0, dir: Down, run: 1});

    let valid = |pos: &(i32, i32)| pos.0 >= 0 && pos.0 < w && pos.1 >= 0 && pos.1 < h;

    while let Some(entry) = stack.pop_front() {
        if !valid(&entry.pos) {
            continue;
        }
        let (x, y) = (entry.pos.0 as usize, entry.pos.1 as usize);
        let cost = entry.cost + (field[y][x] as u32);
        if let Some(cost1) = map.get(&(entry.pos, entry.dir, entry.run)) {
            if cost1 <= &cost {
                continue;
            }
        }
        map.insert((entry.pos, entry.dir, entry.run), cost);
        if entry.run < max_run {
            let dir = entry.dir;
            stack.push_back(StackEntry{pos: dir.displace(entry.pos), cost, dir, run: entry.run + 1});
        }
        if entry.run >= min_run {
            let dir = entry.dir.cw();
            stack.push_back(StackEntry{pos: dir.displace(entry.pos), cost, dir, run: 1});
            let dir = entry.dir.ccw();
            stack.push_back(StackEntry{pos: dir.displace(entry.pos), cost, dir, run: 1});
        }
    }

    let pos = (w - 1, h - 1);
    let mut min = None;
    let mut cmp = |val| {
        if let Some(&val) = val {
            min = min.and_then(|min| Some(std::cmp::min(min, val))).or(Some(val))
        }
    };
    for run in min_run..=max_run {
        cmp(map.get(&(pos, Right, run)));
        cmp(map.get(&(pos, Down, run)));
    }
    min.unwrap()
}
