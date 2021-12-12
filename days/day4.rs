use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

const DAY : usize = 4;

fn main() {
    let testfile = format!("./tests/day{}.in", DAY);
    let workfile = format!("./inputs/day{}.in", DAY);

    let files : Vec<String> = vec![testfile, workfile];
    let names : Vec<String> = vec![String::from("Testfile"), String::from("Workfile")];

    let tmp : Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let items : HashSet<String> = tmp.into_iter().map(|x| x.to_string()).collect();

    // println!("Items: {:?}", items);

    for (f, name) in files.iter().zip(names.iter()) {
        let file = File::open(f).expect("err");
        let bf = BufReader::new(file);
        
        let mut c : usize = 0;
        let mut curr : HashSet<String> = HashSet::new();
        for line in bf.lines().map(|x| x.unwrap()) {
            if line == "" {
                if curr.intersection(&items).collect::<HashSet<&String>>().len() == 7 {
                    c += 1;
                }
                curr = HashSet::new();
            } else {
                let parse : Vec<String> = line.split(' ')
                    .map(|field : &str| field.split(':').next().unwrap().to_string())
                    .collect();

                for val in parse {
                    curr.insert(val);
                }
            }
        }

        if curr.intersection(&items).collect::<HashSet<&String>>().len() == 7 {
            c += 1;
        }

        println!("{}", name);
        println!("Part 1: {}\n", c);
    }
}
