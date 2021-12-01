use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("../input.txt")?;
    println!("{}", part_one(&data));
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
