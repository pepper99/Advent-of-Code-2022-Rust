use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn solve_first(inputs: &Vec<String>) {
    let line = inputs[0].chars().collect::<Vec<char>>();
    let l = line.len();
    for i in 0..l - 3 {
        let mut chars = HashSet::new();
        for j in i..i + 4 {
            chars.insert(line[j]);
        }
        if chars.len() == 4 {
            println!("{}", i + 4);
            break;
        }
    }
}

fn solve_second(inputs: &Vec<String>) {
    let line = inputs[0].chars().collect::<Vec<char>>();
    let l = line.len();
    for i in 0..l - 3 {
        let mut chars = HashSet::new();
        for j in i..i + 14 {
            chars.insert(line[j]);
        }
        if chars.len() == 14 {
            println!("{}", i + 14);
            break;
        }
    }
}

fn get_input(file_name: &str) -> Vec<String> {
    let mut inputs: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(ip) = line {
                inputs.push(ip);
            }
        }
    }
    return inputs;
}

fn main() {
    let inputs = get_input("./d6.txt");
    solve_first(&inputs);
    solve_second(&inputs);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
