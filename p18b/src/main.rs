#![feature(iter_intersperse)]

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let mut pos = (0, 0);
    let mut corners = vec![];
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        // TEST
        /*
        let mut it = line.split(' ');
        let dir = it.next().unwrap().as_bytes()[0] as char;
        let num = it.next().unwrap().parse::<i32>().unwrap();
        let vec = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => panic!("dir {dir}")
        };
        */
        let rest = line.split_once('#').unwrap().1;
        let num = i64::from_str_radix(&rest[0..5], 16).unwrap();
        let dir = rest.as_bytes()[5] as char;
        let vec = match dir {
            '0' => (1, 0),
            '1' => (0, 1),
            '2' => (-1, 0),
            '3' => (0, -1),
            _ => panic!("dir {dir}")
        };
        pos = (pos.0 + num * vec.0, pos.1 + num * vec.1);
        corners.push(pos);
    }
    assert!(pos == (0, 0));

    let mut xs = corners.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    xs.sort();
    xs.dedup();
    let mut ys = corners.iter().map(|(_, y)| *y).collect::<Vec<_>>();
    ys.sort();
    ys.dedup();

    let w = xs.last().unwrap() - xs.first().unwrap() + 1;
    let h = ys.last().unwrap() - ys.first().unwrap() + 1;

    let widths = [0].into_iter()
        .chain(xs.windows(2).map(|s| s[1] - s[0] - 1))
        .chain([0].into_iter())
        .intersperse(1)
        .collect::<Vec<_>>();
    let w0 = widths.len();
    //println!("{widths:?}");

    let heights = [0].into_iter()
        .chain(ys.windows(2).map(|s| s[1] - s[0] - 1))
        .chain([0].into_iter())
        .intersperse(1)
        .collect::<Vec<_>>();
    let h0 = heights.len();
    //println!("{heights:?}");

    let mut field = std::iter::repeat([0u8].repeat(w0)).take(h0).collect::<Vec<_>>();
    let mut last = (0, 0);
    for pos in corners {
        let x1 = xs.iter().position(|x| *x == last.0).unwrap() * 2 + 1;
        let y1 = ys.iter().position(|x| *x == last.1).unwrap() * 2 + 1;
        let x2 = xs.iter().position(|x| *x == pos.0).unwrap() * 2 + 1;
        let y2 = ys.iter().position(|x| *x == pos.1).unwrap() * 2 + 1;
        if x1 == x2 {
            for y in y1..=y2 {
                field[y][x1] = 1u8;
            }
            for y in y2..=y1 {
                field[y][x1] = 1u8;
            }
        } else if y1 == y2 {
            for x in x1..=x2 {
                field[y1][x] = 1u8;
            }
            for x in x2..=x1 {
                field[y1][x] = 1u8;
            }
        }
        last = pos;
    }
    //println!("{field:?}");

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    let mut volume_out = 0;
    while let Some((x, y)) = queue.pop_front() {
        if field[y][x] != 0 {
            continue;
        }
        field[y][x] = 1;
        volume_out += widths[x] * heights[y];
        if x + 1 < w0 { queue.push_back((x + 1, y)); }
        if x > 0 { queue.push_back((x - 1, y)); }
        if y + 1 < h0 { queue.push_back((x, y + 1)); }
        if y > 0 { queue.push_back((x, y - 1)); }
    }
    //println!("{field:?}");

    let volume = w * h - volume_out;
    println!("{volume}");
}
