fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];
    let input = std::fs::read_to_string(filename).unwrap();

    // Part A

    let mut total = 0u64;
    for string in input.split(',') {
        if string.is_empty() {
            continue;
        }
        let mut hash = 0u8;
        for ch in string.trim().as_bytes() {
            hash = hash.wrapping_add(*ch).wrapping_mul(17u8);
        }
        total += hash as u64;
    }
    println!("{total}");

    // Part B

    let mut maps = vec![];
    for _ in 0..256 {
        maps.push(Vec::<(String, usize)>::new());
    }
    for string in input.split(',').map(str::trim) {
        if string.is_empty() {
            continue;
        }
        let mut hash = 0u8;
        let index = 'a: loop {
            for (ix, ch) in string.as_bytes().iter().enumerate() {
                if *ch == b'=' || *ch == b'-' { break 'a ix; }
                hash = hash.wrapping_add(*ch).wrapping_mul(17u8);
            }
        };
        let label = &string[0..index];
        let cmd = string.as_bytes()[index];
        //println!("{hash} {label} {}", cmd as char);

        let map = &mut maps[hash as usize];
        match cmd {
            b'-' => {
                if let Some(pos) = map.iter().position(|(s, _)| s == label) {
                    map.remove(pos);
                }
            }
            b'=' => {
                let val = string[(index + 1)..].parse::<usize>().unwrap();
                if let Some(pos) = map.iter().position(|(s, _)| s == label) {
                    map[pos] = (label.to_owned(), val);
                } else {
                    map.push((label.to_owned(), val));
                }
            }
            _ => panic!("{}", cmd as char)
        }
    }
    let mut total = 0usize;
    for i in 0..256 {
        for (j, (_label, val)) in maps[i].iter().enumerate() {
            let power = (i + 1) * (j + 1) * val;
            //println!("{} {} {} {} => {}", _label, i + 1, j + 1, val, power);
            total += power;
        }
    }
    println!("{total}");
}
