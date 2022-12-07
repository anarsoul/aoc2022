use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_sop(line: &String, count: usize)
{
    let mut sof: Vec<char> = Vec::new();
    let mut idx = 0;
    println!("{}", line);

    for chr in line.chars() {
        sof.push(chr.clone());
        idx += 1;
        if sof.len() == count {
            let mut test = sof.clone();
            test.sort();
            test.dedup();
            if test.len() == count {
                println!("index: {}", idx);
                break;
            }
            sof.remove(0);
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        let line = line.unwrap();
        find_sop(&line, 4);
        find_sop(&line, 14);
    }
}
