use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct Direction {
    delta_x: i32,
    delta_y: i32,
    steps: i32,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let moves = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let tuple = line.split(" ").collect::<Vec<&str>>();
            let dir = match tuple[0] {
                "U" => { Direction{
                        delta_x: 0,
                        delta_y: -1,
                        steps: tuple[1].parse::<i32>().unwrap(),
                    }
                },
                "D" => { Direction{
                        delta_x: 0,
                        delta_y: 1,
                        steps: tuple[1].parse::<i32>().unwrap(),
                    }
                },
                "L" => { Direction{
                        delta_x: -1,
                        delta_y: 0,
                        steps: tuple[1].parse::<i32>().unwrap(),
                    }
                },
                "R" => { Direction{
                        delta_x: 1,
                        delta_y: 0,
                        steps: tuple[1].parse::<i32>().unwrap(),
                    }
                },
                _ => { panic!("Unhandled char: {}", tuple[0])}

            };
            dir
        });

    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    let snake_size = 10;
    // Head is [0], Tail is [snake_size - 1] 
    let mut snake: Vec<(i32, i32)> = vec![(0, 0); snake_size];
    visited.insert((0, 0), true);

    for move_ in moves {
        println!("{:?}", move_);
        for _ in 0..move_.steps {
            let mut head_coords = snake[0];
            head_coords = (head_coords.0 + move_.delta_x, head_coords.1 + move_.delta_y);
            snake[0] = head_coords;

            // Now update rest of the snake
            for idx in 1..snake.len() {
                let head_coords = snake[idx - 1];
                let mut tail_coords = snake[idx]; 
                let mut delta_x = head_coords.0 - tail_coords.0;
                let mut delta_y = head_coords.1 - tail_coords.1;

                if delta_x.abs() < 2 && delta_y.abs() < 2 {
                    continue;
                }

                if delta_x == 0 && delta_y.abs() == 2 ||
                   delta_y == 0 && delta_x.abs() == 2 {
                    delta_x /= 2;
                    delta_y /= 2;
                } else {
                    if delta_x.abs() == 2 {
                        delta_x /= 2;
                    }
                    if delta_y.abs() == 2 {
                        delta_y /= 2;
                    }
                }
                tail_coords = (tail_coords.0 + delta_x, tail_coords.1 + delta_y);
                snake[idx] = tail_coords;
            }
            let tail_coords = snake[snake.len() - 1];
            visited.insert(tail_coords, true);
            println!("Snake: {:?}", snake);
        }
    }
    println!("Visited: {}", visited.keys().len());
}
