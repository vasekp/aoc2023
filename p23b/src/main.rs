use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    Right,
    Left,
    Up,
    Down
}

use Dir::*;

impl Dir {
    fn displace(self, pos: &(usize, usize)) -> (usize, usize) {
        let &(x, y) = pos;
        match self {
            Right => (x + 1, y),
            Left => (x - 1, y),
            Up => (x, y - 1),
            Down => (x, y + 1)
        }
    }

    fn rev(self) -> Dir {
        match self {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up
        }
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
    let (w, h) = (field[0].len(), field.len());

    assert!(field[0][1] == b'.');
    assert!(field[h - 1][w - 2] == b'.');

    let adjacent = |pos: &(usize, usize), dir: Dir| -> u8 {
        let disp = dir.displace(pos);
        field[disp.1][disp.0]
    };

    let mut nodes = vec![];
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if field[y][x] != b'.' {
                continue;
            }
            let pos = (x, y);
            //let neighs = vec![field[y - 1][x], field[y][x - 1], field[y][x + 1], field[y + 1][x]];
            let neighs = [adjacent(&pos, Up), adjacent(&pos, Down), adjacent(&pos, Left), adjacent(&pos, Right)];
            let arrows = neighs.iter().filter(|&&c| "<>^v".find(c as char).is_some()).count();
            let ways = neighs.iter().filter(|&&c| c == b'.').count() + arrows;
            assert!(ways == 2 || arrows >= 2);
            if arrows >= 2 {
                nodes.push((x, y));
            }
        }
    }
    let ul_corner = (1, 0);
    let lr_corner = (w - 2, h - 2);
    nodes.push(lr_corner);
    nodes.push(ul_corner);

    let follow = |pos: &(usize, usize), dir: Dir| -> ((usize, usize), usize) {
        let mut pos = dir.displace(pos);
        let mut dir = dir;
        let mut dist = 1;
        loop {
            for new_dir in [Up, Down, Left, Right] {
                if new_dir == dir.rev() || adjacent(&pos, new_dir) == b'#' {
                    continue;
                }
                dir = new_dir;
                pos = dir.displace(&pos);
                dist += 1;
                break;
            }
            if nodes.contains(&pos) {
                return (pos, dist);
            }
        }
    };

    let mut graph = HashMap::new();
    graph.insert(ul_corner, vec![follow(&ul_corner, Down)]);

    for node in &nodes {
        if node == &lr_corner || node == &ul_corner {
            continue;
        }
        let mut outs = vec![];
        for dir in [Up, Down, Left, Right] {
            if adjacent(node, dir) == b'#' {
                continue;
            }
            outs.push(follow(node, dir));
        }
        graph.insert(*node, outs);
    }
    //println!("{graph:?}");

    let mut queue = VecDeque::new();
    queue.push_back((vec![ul_corner], 1));

    let mut longest = 0;
    while let Some((path, len)) = queue.pop_front() {
        //println!("{path:?} {len}");
        let last = *path.last().unwrap();
        if last == lr_corner {
            longest = std::cmp::max(longest, len);
        } else {
            for (out, dist) in graph.get(&last).unwrap() {
                if path.contains(&out) {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(*out);
                queue.push_back((new_path, len + dist));
            }
        }
    }
    println!("{longest}");
}
