use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut stars = vec![];
    for (y, line) in io::BufReader::new(file).lines().map(Result::unwrap).enumerate() {
        for (x, _) in line.char_indices().filter(|(_, c)| c == &'#') {
            stars.push((x, y));
        }
    }

    const EXPAND: usize = 1000000; // set to 2 for part A

    let mut xs = stars.iter().map(|(x, _)| x).collect::<Vec<_>>();
    let mut xmap = HashMap::new();
    xs.sort();
    xs.dedup();
    let mut tgt = 0usize;
    let mut last = 0usize;
    for x in xs {
        tgt += (x - last) * EXPAND + 1; // This is 1 more than needed but only differences will matter.
        last = x + 1usize;
        xmap.insert(x, tgt as i64);
    }

    let mut ys = stars.iter().map(|(_, y)| y).collect::<Vec<_>>();
    let mut ymap = HashMap::new();
    ys.sort();
    ys.dedup();
    let mut tgt = 0usize;
    let mut last = 0usize;
    for y in ys {
        tgt += (y - last) * EXPAND + 1;
        last = y + 1usize;
        ymap.insert(y, tgt as i64);
    }

    let mut total = 0i64;
    for (x1, y1) in &stars {
        for (x2, y2) in &stars {
            if (x1, y1) >= (x2, y2) { continue; }
            let (x1, y1) = (xmap.get(&x1).unwrap(), ymap.get(&y1).unwrap());
            let (x2, y2) = (xmap.get(&x2).unwrap(), ymap.get(&y2).unwrap());
            total += (x2 - x1).abs() + (y2 - y1).abs();
        }
    }
    println!("{}", total);
}
