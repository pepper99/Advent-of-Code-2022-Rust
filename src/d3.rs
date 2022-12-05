use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;

fn solve_first(inputs: &Vec<String>) {
    let mut priority: Vec<u32> = Vec::new();
    for line in inputs {
        let l: usize = line.len();
        let n: usize = l / 2;
        let first = &line[0..n];
        let second = &line[n..l];
        let second_chars: HashSet<char> = HashSet::from_iter(second.chars());
        for c in first.chars() {
            if second_chars.contains(&c) {
                let mut x = c as u32;
                if x >= 'a' as u32 {
                    x -= 'a' as u32 - 1;
                } else {
                    x -= 'A' as u32 - 27;
                }
                priority.push(x);
                break;
            }
        }
    }
    let ans: u32 = priority.iter().sum();
    println!("{}", ans);
}

fn solve_second(inputs: &Vec<String>) {
    let mut priority: Vec<u32> = Vec::new();
    for i in (0..inputs.len()).step_by(3) {
        let second_chars: HashSet<char> = HashSet::from_iter(inputs[i+1].chars());
        let third_chars: HashSet<char> = HashSet::from_iter(inputs[i+2].chars());
        for c in inputs[i].chars() {
            if second_chars.contains(&c) && third_chars.contains(&c) {
                let mut x = c as u32;
                if x >= 'a' as u32 {
                    x -= 'a' as u32 - 1;
                } else {
                    x -= 'A' as u32 - 27;
                }
                priority.push(x);
                break;
            }
        }
    }
    let ans: u32 = priority.iter().sum();
    println!("{}", ans);
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
    let inputs = get_input("./d3.txt");
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
