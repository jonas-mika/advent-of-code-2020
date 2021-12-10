use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY : usize = 3;

fn main() {
    let testfile : String = format!("./tests/day{}.in", DAY);
    let workfile : String = format!("./inputs/day{}.in", DAY);
    let files : Vec<String> = vec![testfile, workfile];
    let names : Vec<String> = vec![format!("Testing Inputs (Day {})", DAY), 
                                   format!("Working Inputs (Day {})", DAY)];

    for (f,name) in files.iter().zip(names.iter()) {
        let file = File::open(f).expect("file wasn't found.");
        let bf = BufReader::new(file);

        // read into vec<vec<char>>
        let mut forest :  Vec<Vec<char>> = Vec::new();
        for line in bf.lines().map(|line| line.unwrap()) {
            let mut row : Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            forest.push(row);
        }
        
        let x_slopes : Vec<usize> = vec![1, 3, 5, 7, 1];
        let y_slopes : Vec<usize> = vec![1, 1, 1, 1, 2];
        let mut ans : Vec<usize> = Vec::new();

        for (cx, cy) in x_slopes.iter().zip(y_slopes.iter()) {
            let mut x = 0;
            let mut y = 0;
            let mut c = 0;
            while y < forest.len() - 1 {
                // update position 
                y += cy;
                let diff = forest[0].len()-1 - x;
                if diff < *cx {
                    x = cx-diff-1; 
                } else {
                    x += cx;
                }
                if forest[y][x] == '#' {
                    c += 1;
                }
            }
            ans.push(c);
        }

        let ans2 = ans.to_vec();
        let prod = ans2.into_iter().reduce(|a, b| a * b).unwrap();

        println!("{}", name);
        println!("Part 1: {}", ans[1]);
        println!("Part 2: {}\n", prod);
    }
}
