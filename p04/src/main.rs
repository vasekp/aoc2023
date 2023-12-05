use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::VecDeque;

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(format!("Usage: {} file", args[0]));
    }
    let filename = &args[1];

    //let re = Regex::new(r#"Card \+(\d\+): ((\d+) +)*|( +(\d+))*"#).unwrap();
    let re = Regex::new(r#"[0-9]+"#).unwrap();

    let file = File::open(filename).unwrap();
    let mut total = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let split = line.split([':', '|']).collect::<Vec<_>>();
        //let game = re.find(split[0]).unwrap().as_str().parse::<i32>().unwrap();
        let wins = re.find_iter(split[1])
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let guesses_iter = re.find_iter(split[2])
            .map(|m| m.as_str().parse::<i32>().unwrap());
        let mut score = 0;
        for guess in guesses_iter {
            if wins.contains(&guess) {
                score = match score { 0 => 1, s => s*2 };
            }
        }
        //println!("{} {}", game, score);
        total += score;
    }
    println!("{total}");

    let file = File::open(filename).unwrap();
    let mut total = 0;
    let mut copies_pending = VecDeque::new();
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let split = line.split([':', '|']).collect::<Vec<_>>();
        //let game = re.find(split[0]).unwrap().as_str().parse::<i32>().unwrap();
        let copies = copies_pending.pop_front().unwrap_or(0) + 1;
        let wins = re.find_iter(split[1])
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let score = re.find_iter(split[2])
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .filter(|x| wins.contains(x))
            .count();
        if copies_pending.len() < score {
            copies_pending.resize(score, 0);
        }
        for i in 0..score {
            copies_pending[i] += copies;
        }
        //println!("{} {} {} {:?}", game, score, copies, copies_pending);
        total += copies;
    }
    println!("{total}");

    Ok(())
}
