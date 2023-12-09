use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut total = 0i64;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let mut vec = line.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        //vec.reverse(); // <-- uncomment for part B
        for i in 0..vec.len() {
            let i2 = vec.len() - i;
            for j in 0..(i2 - 1) {
                vec[j] = vec[j + 1] - vec[j];
            }
        }
        let next = vec.iter().sum::<i64>();
        //println!("{}", next);
        total += next;
    }
    println!("{total}");
}
