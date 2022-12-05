use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn solve_first(inputs: &Vec<String>) {
    let l = inputs[0].len();
    let n: usize = (l + 1) / 4;
    let mut idx: usize = 0;
    let mut stacks = Vec::<VecDeque<char>>::new();
    for _ in 0..n {
        stacks.push(VecDeque::<char>::new());
    }
    while inputs[idx].as_bytes()[1] != b'1' {
        for j in 0..n {
            if inputs[idx].as_bytes()[j * 4] == b'[' {
                let c = inputs[idx].as_bytes()[j * 4 + 1] as char;
                stacks[j].push_back(c);
            }
        }
        idx += 1;
    }
    idx += 2;
    for i in idx..inputs.len() {
        let mvs: Vec<&str> = inputs[i].split(" ").collect();
        let mut amt = mvs[1].parse::<i32>().unwrap();
        let from = mvs[3].parse::<usize>().unwrap() - 1;
        let to = mvs[5].parse::<usize>().unwrap() - 1;
        while amt > 0 {
            if stacks[from].len() > 0 {
                let from_stack = &mut stacks[from];
                let c = from_stack.pop_front().unwrap();
                let to_stack = &mut stacks[to];
                to_stack.push_front(c);
            }
            amt -= 1;
        }
    }
    for j in 0..n {
        print!("{:}", &stacks[j][0]);
    }
    println!("");
}

fn solve_second(inputs: &Vec<String>) {
    let l = inputs[0].len();
    let n: usize = (l + 1) / 4;
    let mut idx: usize = 0;
    let mut stacks = Vec::<VecDeque<char>>::new();
    for _ in 0..n {
        stacks.push(VecDeque::<char>::new());
    }
    while inputs[idx].as_bytes()[1] != b'1' {
        for j in 0..n {
            if inputs[idx].as_bytes()[j * 4] == b'[' {
                let c = inputs[idx].as_bytes()[j * 4 + 1] as char;
                stacks[j].push_back(c);
            }
        }
        idx += 1;
    }
    idx += 2;
    for i in idx..inputs.len() {
        let mvs: Vec<&str> = inputs[i].split(" ").collect();
        let mut amt = mvs[1].parse::<i32>().unwrap();
        let from = mvs[3].parse::<usize>().unwrap() - 1;
        let to = mvs[5].parse::<usize>().unwrap() - 1;

        let mut adding: Vec<char> = Vec::new();
        while amt > 0 {
            if stacks[from].len() > 0 {
                let from_stack = &mut stacks[from];
                let c = from_stack.pop_front().unwrap();
                adding.push(c);
            }
            amt -= 1;
        }
        for c in adding.iter().rev() {
            let to_stack = &mut stacks[to];
            to_stack.push_front(*c);
        }
    }
    for j in 0..n {
        print!("{:}", &stacks[j][0]);
    }
    println!("");
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
    let inputs = get_input("./d5.txt");
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
