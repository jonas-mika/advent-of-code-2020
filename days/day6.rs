use std::fs::read_to_string;
use std::collections::HashSet;
use std::iter::FromIterator;

const DAY : usize = 6;

pub fn file_to_string(path: &str) -> String {
    let contents = read_to_string(path).unwrap();
    contents
}

fn main()  {
    let file = file_to_string(&format!("./inputs/day{}.in", DAY));
    let chunks: Vec<&str> = file.split("\n\n").collect();

    let mut s1 = 0;
    let mut s2 = 0;
    for chunk in chunks {
        let all_ans = chunk.replace("\n", "");

        let each_ans : Vec<HashSet<char>> = chunk
            .split("\n")
            .map(|person| HashSet::from_iter(person.chars()))
            .filter(|ans| ans.len() > 0)
            .collect();

        
        let tmp : HashSet<char> = HashSet::from_iter(all_ans.chars());
        s1 += tmp.len();

        let intersection : HashSet<char> = each_ans[0]
            .iter()
            .filter(|x| each_ans[1..].iter().all(|set| set.contains(*x)))
            .map(|x| *x)
            .collect();

        s2 += intersection.len();
    }


    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
