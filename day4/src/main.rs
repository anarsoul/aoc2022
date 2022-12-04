use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut contains = 0;
    let mut overlaps = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let caps = re.captures(line.as_str()).unwrap();

        let mut parsed: [i32; 4] = [0; 4];

        for i in 0..4 {
            parsed[i] = caps[i + 1].parse::<i32>().unwrap();
        }

        if parsed[0] <= parsed[2] && parsed[1] >= parsed[3] ||
           parsed[2] <= parsed[0] && parsed[3] >= parsed[1] {
            contains += 1;
        }

        if !(parsed[0] > parsed[3] ||
             parsed[1] < parsed[2]) {
            overlaps += 1;
        }
    }

    println!("Contains: {}", contains);
    println!("Overlaps: {}", overlaps);
}
