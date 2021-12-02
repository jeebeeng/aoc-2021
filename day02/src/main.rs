use std::error::Error;
use std::fs;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("../input.txt")?;
    println!("Part one solution: {}", part_one(&data));
    println!("Part two solution: {}", part_two(&data));
    Ok(())
}

fn part_one(lines: &str) -> i32 {
    let mut hori = 0;
    let mut depth = 0;
    for line in lines.lines() {
        if line.starts_with("forward") {
            let value: i32 = line[line.len()-1..].parse().unwrap();
            hori += value;
        } else if line.starts_with("down") {
            let value: i32 = line[line.len()-1..].parse().unwrap();
            depth += value;
        } else if line.starts_with("up") {
            let value: i32 = line[line.len()-1..].parse().unwrap();
            depth -= value;
        }
    }
    hori * depth
}

fn part_two(lines: &str) -> i32 {
    let mut hori = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines.lines() {
        if line.starts_with("forward") {
            let value: i32 = line[line.len()-1..].parse().unwrap();
            hori += value;
            depth += aim * value;
        } else if line.starts_with("down") {
            let value: i32 = line[line.len()-1..].parse().unwrap();
            aim += value;
        } else if line.starts_with("up") {
            let value: i32 = line[line.len()-1..].parse().unwrap();
            aim -= value;
        }
    }
    hori * depth
}
