use std::fs::File;
use std::io::{BufRead, BufReader};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{map, all_consuming},
    sequence::{preceded, separated_pair},
};
use id_tree::*;
use id_tree::InsertBehavior::*;
use std::collections::HashMap;

#[derive(Debug)]
enum FsEntry {
    Dir(i64, String),
    File(i64, String),
}

#[derive(Debug)]
enum Command {
    Ls(String),
    Cd(String),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    FsEntry(FsEntry),
}

fn parse_ls(line: &str) -> IResult<&str, String> {
    map(tag("ls"), |_| String::from("Ls command"))(line)
}

fn parse_path(line: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| c.is_alphabetic() || "/.".contains(c)),
        String::from
    )(line)
}

fn parse_cd(line: &str) -> IResult<&str, String> {
    map(
        preceded(
            tag("cd "),
            parse_path
        ),
        String::from
    )(line)
}

fn parse_file(line: &str) -> IResult<&str, (i64, String)>
{
    map(
        separated_pair(
            nom::character::complete::i64,
            tag(" "),
            parse_path
        ),
        |x| x,
    )(line)
}

fn parse_dir(line: &str) -> IResult<&str, (i64, String)>
{
    map(
        preceded(
            tag("dir "),
            parse_path
        ),
        |x| (0, x)
    )(line)
}

fn parse_fs_entry(line: &str) -> IResult<&str, FsEntry> {
    alt((
        map(parse_file, |x| FsEntry::File(x.0, x.1)),
        map(parse_dir, |x| FsEntry::Dir(x.0, x.1)),
    ))(line)
}

fn parse_command(line: &str) -> IResult<&str, Command> {
    let (line, _) = tag("$ ")(line)?;
    alt((
        map(parse_ls, Command::Ls),
        map(parse_cd, Command::Cd),
    ))(line)
}

fn parse_line(line: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_fs_entry, Line::FsEntry),
    ))(line)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let res = all_consuming(parse_line)(line.as_str())
                .unwrap()
                .1;
            res
        });

    let mut map: HashMap<(String, NodeId), NodeId> = HashMap::new();
    let mut tree: Tree<FsEntry> = TreeBuilder::new()
        .build();
    let root_id: NodeId = tree.insert(Node::new(FsEntry::Dir(0, "/".into())), AsRoot).unwrap();
    map.insert((String::from("/"), root_id.clone()), root_id.clone());
    let mut cur_id = root_id.clone();
    for line in lines {
        match line {
            Line::Command(cmd) => {
                println!("{:?}", cmd);
                match cmd {
                    Command::Ls(_) => { },
                    Command::Cd(path) => {
                        match path.as_str() {
                            ".." => {
                                cur_id = tree.get(&cur_id).unwrap().parent().unwrap().clone();
                                println!("Going to {}, id: {:?}", path, cur_id);
                            },
                            _ => {
                                cur_id = map[&(path.clone(), cur_id.clone())].clone();
                                println!("Going to {}, id: {:?}", path, cur_id);
                            }
                        }
                    }
                }
            },
            Line::FsEntry(entry) => {
                match entry {
                    FsEntry::Dir(size, name) => {
                        let id: NodeId = tree.insert(
                                Node::new(FsEntry::Dir(size, name.clone())),
                                UnderNode(&cur_id)).unwrap();
                        map.insert((name.clone(), cur_id.clone()), id.clone());
                        println!("Inserted: {}, id: {:?}", name, id);
                    },
                    FsEntry::File(size, name) => {
                        tree.insert(Node::new(FsEntry::File(size, name.clone())),
                                UnderNode(&cur_id)).unwrap();
                        let mut id = cur_id.clone();
                        loop {
                            let dir = tree.get_mut(&id).unwrap();
                            let old_entry = dir.data();
                            if let FsEntry::Dir(old_size, name) = old_entry {
                                dir.replace_data(FsEntry::Dir(old_size + size, name.clone()));
                            }
                            if id == root_id {
                                break;
                            }
                            id = tree.get(&id).unwrap().parent().unwrap().clone();
                        }
                    }
                }
            }
        }
    }

    let mut out = String::new();
    tree.write_formatted(&mut out).unwrap();
    println!("{}", out);
    
    let mut total_size = 0;
    if let FsEntry::Dir(size, _name) = tree.get(&root_id).unwrap().data() {
        total_size = *size;
    }
    let unused = 70000000 - total_size;
    let mut min = total_size;

    let mut sum = 0;
    let mut node = root_id;
    for key in map.keys() {
        let dir = tree.get(&map[key]).unwrap();
        if let FsEntry::Dir(size, name) = dir.data() {
            if *size <= 100000 {
                sum += size;
            }
            if unused + *size >= 30000000 && min > *size {
                println!("Found {}, size {}", name, size);
                min = *size;
                node = map[key].clone();
            } 
        }
    }

    println!("Sum: {}", sum);
    println!("Unused: {}", unused);
    if let FsEntry::Dir(size, name) = tree.get(&node).unwrap().data() {
        println!("Min: {}, size {}", name ,size);
    }
}
