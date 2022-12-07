use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_unique<'a>(str1: &'a Vec<char>, str2: &'a Vec<char>) -> Option<&'a char> {
    for c in str1 {
        if str2.contains(c) {
            return Some(c);
        }
    }
    None
}
fn find_badge<'a>(group: &'a Vec<Vec<char>>) -> Option<&'a char> {
    let mut largest = &group[0];
    for idx in 1..group.len() {
        if group[idx].len() > largest.len() {
            largest = &group[idx];
        }
    }
    for c in largest {
        let mut not_found = false;
        for rucksack in group {
            if !rucksack.contains(c) {
                not_found = true;
                break;
            }
        }
        if !not_found {
            return Some(c);
        }
    }
    None
}

fn char2prio(chr: &char) -> u32 {
    let code = *chr as u32;

    if code >= 'a' as u32 && code <= 'z' as u32 {
        return code - 'a' as u32 + 1;
    }

    if code >= 'A' as u32 && code <= 'Z' as u32 {
        return code - 'A' as u32 + 27;
    }

    return 0;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut total = 0;
    let mut total2 = 0;

    let mut group = Vec::new();
    for line in lines {
        let line: Vec<char> = line.unwrap().chars().collect();
        group.push(line.clone());
        if group.len() == 3 {
            let c = find_badge(&group);
            total2 += char2prio(c.unwrap());
            group.clear();
        }

        let (left, right) = line.split_at(line.len() / 2);
        let left = left.to_vec();
        let right = right.to_vec();
        let c = find_unique(&left, &right);

        println!("{}", c.unwrap());
        total += char2prio(c.unwrap());
    }

    println!("Total: {}", total);
    println!("Total 2: {}", total2);
}
