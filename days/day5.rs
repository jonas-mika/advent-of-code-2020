use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;


const DAY : usize = 5;

fn search(s : &str, mut from : usize, mut to : usize) -> usize {
    for c in s.chars() {
        if c == 'F' || c == 'L' {
            to = (to + from) / 2
        } else if c == 'B' || c == 'R' {
            from = (to+from) / 2
        }
    }

    return to;
}

fn main() {
    let testfile : String = format!("./tests/day{}.in", DAY);
    let workfile : String = format!("./inputs/day{}.in", DAY);
    let files : Vec<String> = vec![testfile, workfile];
    let names : Vec<&str> = vec!["Testfile", "Workfile"];

    for (f, name) in files.iter().zip(names.iter()) {
        let file = File::open(f).expect("error");
        let reader = BufReader::new(file);

        // part 1
        let mut i = 0;
        let mut max_id = 0;
        let mut seats : HashSet<usize>  = HashSet::new();
        for line in reader.lines().map(|line| line.unwrap()) {
            let row : usize = search(&line[..7], 0, 127);
            let seat : usize =  search(&line[7..], 0, 7);

            let id = row * 8 + seat;
            seats.insert(id);

            if id > max_id || i == 0{
                max_id = id;
            }

            i+=1;
        }
        let all_seats : HashSet<usize> = HashSet::from_iter(8..max_id-7);

        println!("{}", name);
        println!("Part 1: {}", max_id);
        println!("Part 2: {:?}\n", (&all_seats - &seats).iter().nth(0).unwrap());
    }
}

