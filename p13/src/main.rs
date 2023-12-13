use std::fs::File;
use std::io::{self, BufRead};

const DIFF: usize = 1; // change to 0 for part A

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut total = 0;
    let mut field = vec![];
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        if !line.is_empty() {
            field.push(line);
        } else {
            total += score(field);
            field = vec![];
        }
    }
    total += score(field);
    println!("{total}");
}

fn score(field: Vec<String>) -> usize {
    100 * reflection(&field) + reflection(&transpose(&field))
}

fn reflection(field: &Vec<String>) -> usize {
    for i in 1..field.len() {
        if std::iter::zip(field[0..i].iter().rev(), field[i..field.len()].iter())
            .map(diff).sum::<usize>() == DIFF {
                return i;
        }
    }
    0
}

fn diff((x, y): (&String, &String)) -> usize {
    std::iter::zip(x.chars(), y.chars())
        .map(|(a, b)| (a != b) as usize)
        .sum()
}

fn transpose(field: &Vec<String>) -> Vec<String> {
    let mut ret = vec![];
    ret.resize(field[0].len(), String::new());
    for line in field {
        for (ix, ch) in line.chars().enumerate() {
            ret[ix].push(ch);
        }
    }
    ret
}
