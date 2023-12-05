use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input").unwrap();
    let mut total = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let digits = line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>();
            if digits.len() < 1 {
                panic!("{}", line);
            }
            let d1 = digits.first().unwrap().to_digit(10).unwrap();
            let d2 = digits.last().unwrap().to_digit(10).unwrap();
            let num = d1 * 10 + d2;
            //println!("{} {}", line, num);
            total += num;
        }
    }
    println!("{}", total);

    const NAMES: [(&str, &str); 9] = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ];

    let file = File::open("input").unwrap();
    let mut total = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(mut line) = line {
            //print!("{} ", line);
            for (name, value) in NAMES {
                // Need to keep the prefix and postfix because of possible overlaps.
                // This is no problem as the string won't be replaced again.
                line = line.replace(name, &(name.to_owned() + value + name));
            }
            let digits = line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>();
            if digits.len() < 1 {
                panic!("{}", line);
            }
            let d1 = digits.first().unwrap().to_digit(10).unwrap();
            let d2 = digits.last().unwrap().to_digit(10).unwrap();
            let num = d1 * 10 + d2;
            //println!("{} {}", line, num);
            total += num;
        }
    }
    println!("{}", total);
}
