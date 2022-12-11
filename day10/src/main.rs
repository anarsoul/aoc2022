use std::fs::File;
use std::io::{BufRead, BufReader};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    combinator::{map, all_consuming},
    sequence::preceded,
};

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn cycle_nr(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

fn parse_noop(line: &str) -> IResult<&str, Instruction>
{
    map(tag("noop"), |_| Instruction::Noop)(line)
}

fn parse_addx(line: &str) -> IResult<&str, i32>
{
    map(
        preceded(
            tag("addx "),
            nom::character::complete::i32,
        ),
        |x| x
    )(line)
}

fn parse_line(line: &str) -> IResult<&str, Instruction> {
    alt((
        map(parse_noop, |x| x),
        map(parse_addx, Instruction::AddX),
    ))(line)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut instructions = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let res = all_consuming(parse_line)(line.as_str())
                .unwrap()
                .1;
            res
        });

    let mut cycle = 1;
    let mut instr = instructions.next().unwrap();
    let base = 20;
    let stride = 40;
    let last = 241;
    let mut sum = 0;
    let mut x = 1;
    let mut str = String::from("");
    let mut pos = 0;
    loop {
        //println!("Instr: {:?}", instr);
        for instr_cycle in 0..instr.cycle_nr() {
            match cycle {
                _ => {}
            };
            let diff: i32 = pos - x;
            if diff.abs() <  2 {
                str.push('#');
            } else {
                str.push('.');
            }
            pos += 1;
            if pos == 40 {
                println!("{}", str);
                str = String::from("");
                pos = 0;
            }
            cycle += 1;
            if instr_cycle == instr.cycle_nr() - 1 {
                match instr {
                    Instruction::Noop => {},
                    Instruction::AddX(val) => { x += val}
                }
            }
            if (cycle - base) % stride == 0 {
                sum += cycle * x;
            } 
        }
        if cycle >= last {
            break;
        }
        instr = instructions.next().unwrap();
    }
    println!("Sum is {}", sum);
}
