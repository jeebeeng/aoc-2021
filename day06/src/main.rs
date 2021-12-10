use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("../input.txt")?;
    println!("Part one solution: {}", part_one(&data));
    println!("Part two solution: {}", part_two(&data));
    Ok(())
}

fn part_one(lines: &str) -> i32 {
    let mut list: Vec<i32> = lines
        .replace("\n", "")
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();

    for _ in 0..80 {
        for j in 0..list.len() {
            if list[j] == 0 {
                list[j] = 6;
                list.push(8);
            } else {
                list[j] -= 1;
            }
        }
    }

    list.len().try_into().unwrap()
}

fn part_two(lines: &str) -> i32 {
    let mut list: Vec<i32> = lines
        .replace("\n", "")
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();

    for _ in 0..256 {
        for j in 0..list.len() {
            if list[j] == 0 {
                list[j] = 6;
                list.push(8);
            } else {
                list[j] -= 1;
            }
        }
    }

    list.iter().sum()
}
