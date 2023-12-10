use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;
use num::integer::lcm;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut map = HashMap::new();
    let mut lines = io::BufReader::new(file).lines().map(Result::unwrap);
    let instructions = lines.next().unwrap();
    assert!(lines.next().unwrap().is_empty());
    let re = Regex::new("([A-Z]*) = \\(([A-Z]*), ([A-Z]*)\\)").unwrap();
    for line in lines {
        let cap = re.captures(&line).unwrap();
        let (node, left, right) = (&cap[1], &cap[2], &cap[3]);
        map.insert(node.to_owned(), [left.to_owned(), right.to_owned()]);
    }

    let duration = |mut pos| -> usize {
        for (n, dir) in std::iter::repeat_with(|| instructions.chars())
            .flatten()
            .map(|d| match d { 'L' => 0, 'R' => 1, _ => panic!("{d}") })
            .enumerate() {
                pos = &map.get(pos).unwrap()[dir];
                if pos == "ZZZ" {
                    return n + 1;
                }
            }
        panic!();
    };

    println!("{}", duration(&"AAA".to_owned()));

    let next_z = |pos: &String| -> (usize, String) {
        let mut pos = pos;
        for (n, dir) in std::iter::repeat_with(|| instructions.chars())
            .flatten()
            .map(|d| match d { 'L' => 0, 'R' => 1, _ => panic!("{d}") })
            .enumerate() {
                pos = &map.get(pos).unwrap()[dir];
                if &pos[2..3] == "Z" {
                    return (n + 1, pos.clone());
                }
            }
        panic!();
    };

    let mut total = 1u64;
    for key in map.keys().filter(|k| &k[2..3] == "A") {
        let (dur, z) = next_z(&key);
        // check: z reached at instruction string boundary
        assert!(dur % instructions.len() == 0);
        // check: the same z next reached with the same period
        assert!(next_z(&z).0 == dur);
        assert!(next_z(&z).1 == z);
        // sufficient conditions for LCM
        total = lcm(total, dur as u64);
    }
    println!("{total}");
}
