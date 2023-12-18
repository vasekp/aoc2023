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
    let mut dug = vec![];
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
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
        for _ in 1..=num {
            pos = (pos.0 + vec.0, pos.1 + vec.1);
            dug.push(pos);
        }
    }
    assert!(pos == (0, 0));

    let min_x = *dug.iter().map(|(x, _)| x).reduce(std::cmp::min).unwrap();
    let max_x = *dug.iter().map(|(x, _)| x).reduce(std::cmp::max).unwrap();
    let min_y = *dug.iter().map(|(_, y)| y).reduce(std::cmp::min).unwrap();
    let max_y = *dug.iter().map(|(_, y)| y).reduce(std::cmp::max).unwrap();
    //println!("{min_x} {max_x} {min_y} {max_y}");

    let w = (max_x - min_x + 3) as usize;
    let h = (max_y - min_y + 3) as usize;
    let mut field = vec![];
    for _ in 0..h {
        field.push([b'.'].repeat(w));
    }
    for (x, y) in dug {
        field[(y - min_y + 1) as usize][(x - min_x + 1) as usize] = b'#';
    }

    /*for line in field {
        println!("{}", std::str::from_utf8(&line).unwrap());
    }*/

    // Flood fill
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    let mut volume_out = 0;
    while let Some((x, y)) = queue.pop_front() {
        if x < 0 || x >= (w as i32) || y < 0 || y >= (h as i32) || field[y as usize][x as usize] != b'.' {
            continue;
        }
        field[y as usize][x as usize] = b'_';
        volume_out += 1;
        queue.push_back((x + 1, y));
        queue.push_back((x - 1, y));
        queue.push_back((x, y + 1));
        queue.push_back((x, y - 1));
    }

    /*for line in field {
        println!("{}", std::str::from_utf8(&line).unwrap());
    }*/

    let volume = w * h - volume_out;
    println!("{volume}");
}
