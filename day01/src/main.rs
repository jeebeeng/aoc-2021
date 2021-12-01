use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("../input.txt")?;
    println!("Part one solution: {}", part_one(&data));
    println!("Part two solution: {}", part_two(&data));
    Ok(())
}

fn part_one(lines: &str) -> i32 {
    let mut i = 0;
    let mut count = 0;
    let mut prev = 0;

    for line in lines.lines() {
        let line: i32 = line.parse().unwrap();

        if i > 0 {
            count += (line > prev) as i32; 
        }
        prev = line;
        i += 1;
    }
    count
}

fn part_two(lines: &str) -> i32 {
    let mut count = 0;

    let lines: Vec<i32> = lines.lines().map(|line| {
        line.parse::<i32>().unwrap()
    }).collect();

    for i in 0..lines.len() {
        if i > 2 {
            let prev = lines[i-3] + lines[i-2] + lines[i-1];
            let curr = prev - lines[i-3] + lines[i];
            count += (curr > prev) as i32;          
        }
    }

    count
}
