//use std::collections::VecDeque;
use std::io::{self, BufRead};
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let mut vert = vec![];
    let file = std::fs::File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let (from, tos) = line.split_once(": ").unwrap();
        let from = from.to_owned();
        if !vert.contains(&from) { vert.push(from); }
        for to in tos.split(' ') {
            let to = to.to_owned();
            if !vert.contains(&to) { vert.push(to.to_owned()); }
        }
    }

    let size = vert.len();

    let mut edges = vec![];
    let file = std::fs::File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let (from, tos) = line.split_once(": ").unwrap();
        let from_ix = vert.iter().position(|s| s == from).unwrap();
        for to in tos.split(' ') {
            let to_ix = vert.iter().position(|s| s == to).unwrap();
            edges.push((from_ix, to_ix));
        }
    }

    let mut rng = rand::thread_rng();
    loop {
        let (conn, w1, w2) = karger(size, &edges, &mut rng);
        if conn == 3 {
            println!("{}", w1 * w2);
            break;
        }
    }
}

fn karger(size: usize, edges: &[(usize, usize)], rng: &mut impl Rng) -> (usize, usize, usize) {
    let mut edges = edges.to_owned();
    let mut weights = [1].repeat(size);
    let mut size = size;
    edges.shuffle(rng);
    let edges_len = edges.len();
    for i in 0..edges_len {
        let (from, to) = edges[i];
        if from == to { continue; }
        for edge in &mut edges {
            if edge.0 == from { edge.0 = to; }
            if edge.1 == from { edge.1 = to; }
        }
        weights[to] += weights[from];
        size -= 1;
        if size == 2 {
            break;
        }
    }

    edges.retain(|(x, y)| x != y);
    let conn = edges.len();
    let &(p1, p2) = edges.first().unwrap();
    (conn, weights[p1], weights[p2])
}
