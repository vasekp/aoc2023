use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(format!("Use {} input_file", args[0]));
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut total = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let parts = line.split(": ").collect::<Vec<_>>();
        let mut it = parts[0].split(" ");
        assert_eq!(it.next().unwrap(), "Game");
        let id: i32 = it.next().unwrap().parse().unwrap();
        //println!("{id}");

        let counts = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);
        let mut possible = true;
        for play in parts[1].split("; ") {
            for rec in play.split(", ") {
                let syms = rec.split(" ").collect::<Vec<_>>();
                let tpe = &syms[1];
                let exp: i32 = syms[0].parse().unwrap();
                let max = *counts.get(tpe).unwrap();
                if exp > max {
                    println!("Game {}: {} {} > {}", id, tpe, exp, max);
                    possible = false;
                }
            }
        }

        if possible {
            total += id;
        }
    }
    println!("{}", total);

    let file = File::open(filename).unwrap();
    let mut total = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let parts = line.split(": ").collect::<Vec<_>>();
        /*let mut it = parts[0].split(" ");
        assert_eq!(it.next().unwrap(), "Game");
        let id: i32 = it.next().unwrap().parse().unwrap();*/
        //println!("{id}");

        let mut counts = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);
        for play in parts[1].split("; ") {
            for rec in play.split(", ") {
                let syms = rec.split(" ").collect::<Vec<_>>();
                let tpe = &syms[1];
                let count: i32 = syms[0].parse().unwrap();
                let max = counts.get_mut(tpe).unwrap();
                if count > *max {
                    *max = count;
                }
            }
        }
        //print!("Game {id}: ");
        //for (tpe, count) in counts {
        //    print!("{} {}, ", count, tpe);
        //}
        //println!("");
        let mut power = 1;
        for (_tpe, count) in counts {
            power *= count;
        }
        total += power;
    }
    println!("{}", total);

    Ok(())
}
