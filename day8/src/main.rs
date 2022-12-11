use std::fs::File;
use std::io::{BufRead, BufReader};

fn walk(matrix: &Vec<Vec<i32>>,
        x: i32, y: i32,
    x_inc: i32, y_inc: i32) -> i32
{
    assert!(x_inc == 0 && y_inc != 0 || x_inc != 0 && y_inc == 0);
    let mut x2 = x + x_inc;
    let mut y2 = y + y_inc;
    let mut score = 0;

    while x2 >= 0 && x2 < matrix.len() as i32 && y2 >= 0 && y2 < matrix.len() as i32 {
        score += 1;
        if matrix[y as usize][x as usize] <= matrix[y2 as usize][x2 as usize] {
            break;
        } 
        x2 += x_inc;
        y2 += y_inc;
    }

    score
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let matrix = reader
        .lines()
        .map(|line| {
            line
                .unwrap()
                .chars()
                .map(|c| c.to_string()
                          .parse::<i32>()
                          .unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut visible: Vec<Vec<i32>> = Vec::new();
    for _ in 0..matrix.len() {
        visible.push(vec![0; matrix.len()]);
    }

    // Left to right and right to left passes
    for y in 0..matrix.len() {
        let mut prev_lr = -1;
        let mut prev_rl = -1;
        for x in 0..matrix.len() {
            if matrix[y][x] > prev_lr {
                visible[y][x] = 1;
                prev_lr = matrix[y][x];
            }
            let x_rl = matrix.len() - 1 - x;
            if matrix[y][x_rl] > prev_rl {
                visible[y][x_rl] = 1;
                prev_rl = matrix[y][x_rl];
            } 
        }
    }

    // Top to bottom and bottom to top passes
    for x in 0..matrix.len() {
        let mut prev_tb = -1;
        let mut prev_bt = -1;
        for y in 0..matrix.len() {
            if matrix[y][x] > prev_tb {
                visible[y][x] = 1;
                prev_tb = matrix[y][x];
            }
            let y_bt = matrix.len() - 1 - y;
            if matrix[y_bt][x] > prev_bt {
                visible[y_bt][x] = 1;
                prev_bt = matrix[y_bt][x];
            }
        }
    }

    let sum = visible
                .iter()
                .fold(0, |acc, x| {
                    acc + x.iter().sum::<i32>()
                });

    println!("Trees visible: {}", sum);

    let mut max_score = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix.len() {
            let top = walk(&matrix, x as i32, y as i32, 0, -1);
            let bottom = walk(&matrix, x as i32, y as i32, 0, 1);
            let left = walk(&matrix, x as i32, y as i32, -1, 0);
            let right = walk(&matrix, x as i32, y as i32, 1, 0);
            let score = left * right * top * bottom;
            if score > max_score {
                max_score = score;
            }
        }
    }
    
    println!("Max score: {}", max_score);

}
