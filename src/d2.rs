use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn check_win(opp: &str, you: &str) -> i32 {
    let mut score = 0;
    if you == "X" {
        score += 1;
        if opp == "A" {
            score += 3
        }
        else if opp == "C" {
            score += 6
        }
    }
    else if you == "Y" {
        score += 2;
        if opp == "A" {
            score += 6
        }
        else if opp == "B" {
            score += 3
        }
    }
    else if you == "Z" {
        score += 3;
        if opp == "B" {
            score += 6
        }
        else if opp == "C" {
            score += 3
        }
    }
    return score;
}

fn check_outcome(opp: &str, you: &str) -> i32 {
    let mut score = 0;
    if you == "X" {
        if opp == "A" {
            score += 3
        }
        else if opp == "B" {
            score += 1
        }
        else if opp == "C" {
            score += 2
        }
    }
    else if you == "Y" {
        score += 3;
        if opp == "A" {
            score += 1
        }
        else if opp == "B" {
            score += 2
        }
        else if opp == "C" {
            score += 3
        }
    }
    else if you == "Z" {
        score += 6;
        if opp == "A" {
            score += 2
        }
        else if opp == "B" {
            score += 3
        }
        else if opp == "C" {
            score += 1
        }
    }
    return score;
}

fn main() {
    let mut scoreW = 0;
    let mut scoreO = 0;
    if let Ok(lines) = read_lines("./d2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(" ");
                let vec: Vec<&str> = split.collect();
                scoreW += check_win(vec[0], vec[1]);
                scoreO += check_outcome(vec[0], vec[1]);
            }
        }
        println!("{}", scoreW);
        println!("{}", scoreO);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}