use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut stacks: Vec<Vec<String>> = Vec::new();
    let mut lines = reader.lines();

    for line in &mut lines {
        let line = line.unwrap();
        if stacks.is_empty() {
            for _ in 0..((line.len() + 1) / 4) {
                stacks.push(Vec::new());
            }
        }
        let vc: Vec<char> = line.chars().collect();
        for idx in 0..((vc.len() + 1) / 4) {
            if vc[idx * 4] == ' ' {
                continue
            }
            stacks[idx].insert(0, vc[idx * 4 + 1].to_string());
        }
        println!("{}", line);
        if line.is_empty() {
            break;
        }
    }

    let mut stacks2 = stacks.clone();

    println!("CrateMover 9000:");
    for stack in &stacks {
        println!("{:?}", stack);
    }
    println!("------");
    
    println!("CrateMover 9001:");
    for stack in &stacks2 {
        println!("{:?}", stack);
    }
    println!("------");
    
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in &mut lines {
        let line = line.unwrap();
        let caps = re.captures(line.as_str()).unwrap();

        let count = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap() - 1;
        let to = caps[3].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let item = stacks[from].pop().unwrap();
            stacks[to].push(item);
        }

        // I don't like it either, but I didn't find a way
        // to split a vector into two mutable vectors
        let mut items = Vec::new();
        for _ in 0..count {
            items.insert(0, stacks2[from].pop().unwrap());
        }
        stacks2[to].append(&mut items);
    }

    println!("CrateMover 9000:");
    for stack in &stacks {
        println!("{:?}", stack);
    }
    println!("------");
    
    println!("CrateMover 9001:");
    for stack in &stacks2 {
        println!("{:?}", stack);
    }
    println!("------");

    let mut res1 = String::new();
    for stack in &stacks {
        res1.push_str(&stack[stack.len() - 1]);
    }
    let mut res2 = String::new();
    for stack in &stacks2 {
        res2.push_str(&stack[stack.len() - 1]);
    }
    println!("Res1: {}", res1);
    println!("Res1: {}", res2);
}
