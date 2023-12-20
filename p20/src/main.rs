use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};
use std::cell::{Cell, RefCell};

#[derive(Debug)]
enum GateType {
    Fanout,
    Flip,
    Nand(RefCell<Vec<String>>)
}

#[derive(Debug)]
struct Gate {
    tpe: GateType,
    out: Vec<String>,
    value: Cell<bool>
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let mut map = HashMap::new();
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let (src, dst) = line.split_once(" -> ").unwrap();
        let prefix = &src[0..1];
        let (name, tpe) = match prefix {
            "&" => (&src[1..], GateType::Nand(RefCell::new(vec![]))),
            "%" => (&src[1..], GateType::Flip),
            "b" => (src, GateType::Fanout),
            _ => panic!("src {src}")
        };
        let out = dst.split(", ").map(str::to_owned).collect::<Vec<_>>();
        let gate = Gate{tpe, out, value: Cell::new(false)};
        map.insert(name.to_owned(), gate);
    }

    for (name, gate) in &map {
        for out in &gate.out {
            let Some(gate) = map.get(out) else { continue; };
            if let GateType::Nand(ins) = &gate.tpe {
                ins.borrow_mut().push(name.clone());
            }
        }
    }
    //println!("{map:?}");

    let mut queue = VecDeque::new();
    let mut count = [0u64, 0u64];

    for _ in 1..=1000 {
        queue.push_back(("broadcaster", false));
        while let Some((name, pulse)) = queue.pop_front() {
            //println!("{name} {pulse}");
            count[pulse as usize] += 1;
            let Some(gate) = map.get(name) else { continue; };
            let value = match &gate.tpe {
                GateType::Fanout => pulse,
                GateType::Flip => if !pulse { !gate.value.get() } else { continue; }
                GateType::Nand(ins) => !ins.borrow().iter().all(|s| map.get(s).unwrap().value.get())
            };
            gate.value.replace(value);
            for out in &gate.out {
                queue.push_back((out, value));
            }
        }
        //println!("{count:?}");
        //println!("{map:?}");
    }
    //println!("{count:?}");
    println!("{}", count[0] * count[1]);
}
