use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut total = 0;
    let mut max: Vec<i32> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            if max.len() < 3 {
                max.push(total);
                total = 0;
                continue;
            }
            if total <= max[0] {
                total = 0;
                continue;
            }
            max.remove(0);
            max.push(total);
            max.sort();
            total = 0;
            continue;
        }

        total += line.parse::<i32>().unwrap();
    }
    println!("Max is {:?}, sum: {}", max, max.iter().sum::<i32>());
}
