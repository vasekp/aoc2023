use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(format!("Use {} input_file", args[0]));
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let schem = io::BufReader::new(file).lines().map(Result::unwrap).collect::<Vec<_>>();

    let mut total = 0;
    let regex = Regex::new(r#"(\d)+"#).unwrap();
    for (i, line) in schem.iter().enumerate() {
        let llen = line.len();
        for m in regex.find_iter(&line) {
            //println!("{i} {}: {} {}", m.as_str(), m.start(), m.len());
            let num = m.as_str().parse::<i32>().unwrap();
            let start = m.start();
            let end = start + m.len();
            let from = match start { 0 => 0, i => i - 1 };
            let to = match end { l if l == llen => l, l => l + 1 };
            let mut sym = None;
            if i > 0 {
                sym = sym.or_else(|| scan(&schem[i - 1][from..to]));
            }
            if i < schem.len() - 1 {
                sym = sym.or_else(|| scan(&schem[i + 1][from..to]));
            }
            if start > 0 {
                sym = sym.or_else(|| scan(&line[(start - 1)..start]));
            }
            if end < llen {
                sym = sym.or_else(|| scan(&line[end..(end + 1)]));
            }
            if let Some(_s) = sym {
                //println!("found {s}");
                total += num;
            } else {
                //println!("none");
            }
        }
    }
    println!("{total}");

    //let mut total = 0;
    let mut map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    for (i, line) in schem.iter().enumerate() {
        let llen = line.len();
        for m in regex.find_iter(&line) {
            //println!("{i} {}: {} {}", m.as_str(), m.start(), m.len());
            let num = m.as_str().parse::<i32>().unwrap();
            let start = m.start();
            let end = start + m.len();
            let from_line = match i { 0 => 0, l => l - 1 };
            let to_line = match i { i if i == llen - 1 => i + 1, i => i + 2 };
            let from_char = match start { 0 => 0, i => i - 1 };
            let to_char = match end { l if l == llen => l, l => l + 1 };
            for (li, line2) in schem[from_line..to_line].iter().enumerate() {
                for (ci, chr) in line2[from_char..to_char].chars().enumerate() {
                    if chr == '*' {
                        //println!("{}: gear at {}, {}", num, from_line + li, from_char + ci);
                        let coords = (from_line + li, from_char + ci);
                        map.entry(coords)
                            .and_modify(|v| v.push(num))
                            .or_insert(vec![num]);
                    }
                }
            }
        }
    }
    //println!("{:?}", map);
    let mut total = 0;
    for (entry, vec) in map.iter() {
        if vec.len() == 2 {
            let mut prod = 1;
            for val in vec { prod *= val; }
            total += prod;
        }
    }
    println!("{total}");

    Ok(())
}

fn scan(text: &str) -> Option<char> {
    let is_sym = |c: &char| !c.is_ascii_digit() && c != &'.';
    text.chars().filter(is_sym).collect::<Vec<_>>().first().copied()
}
