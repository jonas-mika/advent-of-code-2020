use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY : usize = 2;

#[derive(Debug)]
struct PW {
    min: usize,
    max: usize,
    letter: char,
    pw: String,
}

/*
// debugging
impl PW {
    pub fn new(min: usize, max: i32, letter: char, pw: String) -> Self {
        PW { min, max, letter, pw } 
    }

    pub fn to_string(&self) -> String {
        format!("PW(min: {}, max: {}, letter: {}, pw: {})", &self.min, &self.max, &self.letter, &self.pw)
    }
}
*/


fn main() {
    let testfile : String = format!("./tests/day{}.in", DAY);
    let workfile : String = format!("./inputs/day{}.in", DAY);
    let files : Vec<String> = vec![testfile, workfile];
    let names : Vec<String> = vec![format!("Testing Inputs (Day {})", DAY), 
                                   format!("Working Inputs (Day {})", DAY)];

    for (f, name) in files.iter().zip(names.iter()) {
        let file = File::open(f).expect("file wasn't found.");
        let reader = BufReader::new(file);

        let mut p1 = 0;
        let mut p2 = 0;

        for line in reader.lines().map(|line| line.unwrap()) {
            let parse: Vec<&str> = line.split(' ').collect();

            let nums: Vec<usize> = parse[0].split('-').map(|x| x.parse().unwrap()).collect();
            let min = &nums[0];
            let max = &nums[1];
            let letter = &parse[1].chars().nth(0).unwrap();

            let crack = PW {
                min: *min,
                max: *max,
                letter: *letter, 
                pw: parse[2].to_string(),
            };


            let mut tmp = 0;
            for c in crack.pw.chars() {
                if c == crack.letter {
                    tmp += 1;
                } 
            }

            // valid password for part 1 (number of specified char inside min-max interval)
            if crack.min <= tmp && tmp <= crack.max {
                p1 += 1
            }

            let mut tmp = 0;
            if crack.letter == crack.pw.chars().nth(crack.min-1).unwrap() {
                tmp += 1
            }
            if crack.letter == crack.pw.chars().nth(crack.max-1).unwrap() {
                tmp += 1
            }

            // valid password for part 2 (letter appears exactly ones at min/ max pos)
            if tmp == 1 {
                p2 += 1;
            }
        }
        println!("{}", name);
        println!("Part 1: {}", p1);
        println!("Part 2: {}\n", p2);
    }

}
