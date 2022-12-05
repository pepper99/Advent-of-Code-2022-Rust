use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn solve_first(inputs: &Vec<String>) {
    let mut ans = 0;
    for line in inputs {
        let vec: Vec<&str> = line.split(",").collect();
        let a: Vec<u32> = vec[0].split("-").map(|x| x.parse().unwrap()).collect();
        let b: Vec<u32> = vec[1].split("-").map(|x| x.parse().unwrap()).collect();
        if (a[0] <= b[0] && a[1] >= b[1]) || (a[0] >= b[0] && a[1] <= b[1]) {
            ans += 1
        }
    }
    println!("{}", ans);
}

fn solve_second(inputs: &Vec<String>) {
    let mut ans = 0;
    for line in inputs {
        let vec: Vec<&str> = line.split(",").collect();
        let a: Vec<u32> = vec[0].split("-").map(|x| x.parse().unwrap()).collect();
        let b: Vec<u32> = vec[1].split("-").map(|x| x.parse().unwrap()).collect();
        if (a[0] >= b[0] && a[0] <= b[1]) || (a[1] >= b[0] && a[0] <= b[1]) {
            ans += 1
        }
    }
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
    let inputs = get_input("./d4.txt");
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
