use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone)]
enum Result {
    Lose,
    Draw,
    Win,
}

impl Shape {
    fn to_score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn p2shape(p1: &Shape, res: &Result) -> Shape {
    match p1 {
        Shape::Rock => {
            match res {
                Result::Lose => Shape::Scissors,
                Result::Draw => Shape::Rock,
                Result::Win => Shape::Paper,
            }
        },
        Shape::Paper => {
            match res {
                Result::Lose => Shape::Rock,
                Result::Draw => Shape::Paper,
                Result::Win => Shape::Scissors,
            }
        },
        Shape::Scissors => {
            match res {
                Result::Lose => Shape::Paper,
                Result::Draw => Shape::Scissors,
                Result::Win => Shape::Rock,
            }
        }
    }
}

fn round(p1: &Shape, p2: &Shape) -> i32 {
    match p1 {
        Shape::Rock => {
            match p2 {
                Shape::Rock => 3,
                Shape::Paper => 6,
                Shape::Scissors => 0,
            }
        },
        Shape::Paper => {
            match p2 {
                Shape::Rock => 0,
                Shape::Paper => 3,
                Shape::Scissors => 6,
            }
        },
        Shape::Scissors => {
            match p2 {
                Shape::Rock => 6,
                Shape::Paper => 0,
                Shape::Scissors => 3,
            }
        },
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut player_a = HashMap::new();
    let mut player_b = HashMap::new();
    let mut result = HashMap::new();

    player_a.insert('A', Shape::Rock);
    player_a.insert('B', Shape::Paper);
    player_a.insert('C', Shape::Scissors);
    player_b.insert('X', Shape::Rock);
    player_b.insert('Y', Shape::Paper);
    player_b.insert('Z', Shape::Scissors);
    result.insert('X', Result::Lose);
    result.insert('Y', Result::Draw);
    result.insert('Z', Result::Win);

    let mut total = 0;
    let mut total2 = 0;
    for line in lines {
        let line = line.unwrap();

        let vc: Vec<char> = line.chars().collect();
        let p1 = &player_a[&vc[0]];
        let p2 = &player_b[&vc[2]];
        let res = &result[&vc[2]];

        println!("Part #1: p1: {:?}, p2: {:?}, round: {}", p1, p2, round(p1, p2));  
        total += p2.to_score();
        total += round(p1, p2);

        let p2 = p2shape(p1, res);

        println!("Part #1: p1: {:?}, p2: {:?}, round: {}", p1, &p2, round(p1, &p2));
        total2 += &p2.to_score();
        total2 += round(p1, &p2);
    }

    println!("Part #1 total score: {}", total);
    println!("Part #2 total score: {}", total2);
}
