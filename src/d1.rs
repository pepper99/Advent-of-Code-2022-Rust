use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./d1.txt") {
        let mut v: Vec<i32> = vec![0];
        let mut it: usize = 0;
        for line in lines {
            if let Ok(x) = line {
                if x != "" {
                    v[it] = v[it] + x.parse::<i32>().unwrap();
                }
                else {
                    it += 1;
                    v.push(0);
                }
            }
        }
        v.sort_by(|a, b| b.cmp(a));
        println!("{}", v[0]);
        println!("{}", v[0] + v[1] + v[2]);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
