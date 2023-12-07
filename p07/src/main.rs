use std::fs::File;
use std::io::{self, BufRead};

const CARDS: usize = 5;
const TYPES: usize = 13;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut hands = vec![];
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let values = map_values(&line[0..CARDS]);
        let bid = line.split_once(' ').unwrap().1.parse::<i64>().unwrap();
        let hand_type = evaluate(&values);
        //let value = hand_type * 16u8 + values[0];
        //println!("{} {}", &line[0..CARDS], value);
        let mut sort_key: [u8; CARDS + 1] = Default::default();
        sort_key[0] = hand_type;
        for i in 0..CARDS {
            sort_key[i + 1] = values[i];
        }
        hands.push((sort_key, bid));
    }
    hands.sort_by_key(|x| x.0);
    let mut total = 0i64;
    for (rank, (_, bid)) in hands.iter().enumerate() {
        //println!("{} {}", bid, rank + 1);
        total += bid * ((rank + 1) as i64);
    }
    println!("{total}");
}

fn map_values(s: &str) -> [u8; CARDS] {
    let s = s.as_bytes();
    let mut ret: [u8; CARDS] = Default::default();
    for i in 0..CARDS {
        ret[i] = map_card(s[i]);
    }
    ret
}

fn map_card(c: u8) -> u8 {
    (match c {
        x @ b'2'..=b'9' => x - b'0',
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!("card {c}")
    }) - 2u8
}

fn evaluate(values: &[u8; CARDS]) -> u8 {
    let mut counts: [u8; TYPES] = Default::default();
    for v in values {
        counts[*v as usize] += 1;
    }
    let mut pairs = counts.into_iter().enumerate().collect::<Vec<_>>();
    pairs.sort_by_key(|x| x.1);
    pairs.reverse();
    match (pairs[0].1, pairs[1].1) {
        (5, 0) => 6,
        (4, 1) => 5,
        (3, 2) => 4,
        (3, 1) => 3,
        (2, 2) => 2,
        (2, 1) => 1,
        _ => 0
    }
}
