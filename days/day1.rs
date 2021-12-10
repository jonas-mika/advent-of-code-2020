use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY : i32 = 1;

fn part1(vec: &Vec<i32>) -> i32{
    // two sum 
    for x in vec {
        for y in vec {
            if x+y == 2020 {
                return x * y;
            }
        }
    }
    return 0;
}


fn part2(vec: &Vec<i32>) -> i32{
    // two sum 
    for x in vec {
        for y in vec {
            for z in vec {
                if x+y+z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    return 0;
}

fn main()  {
    let testfile : String = format!("./tests/day{}.in", DAY);
    let workfile : String = format!("./inputs/day{}.in", DAY);
    let files : Vec<String> = vec![testfile, workfile];
    let names : Vec<String> = vec![format!("Testing Inputs (Day {})", DAY), 
                                   format!("Working Inputs (Day {})", DAY)];

    for (f, name) in files.iter().zip(names.iter()) {
        let file = File::open(f).expect("file wasn't found.");
        let reader = BufReader::new(file);

        let vals: Vec<i32> = reader
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect();


        println!("{}", name);
        println!("Part 1: {}", part1(&vals));
        println!("Part 2: {}\n", part2(&vals));
    }
}
