use std::error::Error;
use std::fs;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("../input.txt")?;
    println!("Part one solution: {}", part_one(&data));
    println!("Part two solution: {}", part_two(&data));
    Ok(())
}

fn part_one(lines: &str) -> isize {
    let len = lines.lines().next().unwrap().len();
    let mut count = vec![(0, 0); len];
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for line in lines.lines() {
        for i in 0..line.len() {
            match line.as_bytes()[i] {
               48 => { count[i].0 += 1 } 
               _ => { count[i].1 += 1 }
            }
        }       
    }

    for pair in count {
        if pair.0 > pair.1 {
            gamma.push_str("0");
            epsilon.push_str("1");
        } else {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
    }  
    
    isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
}

fn part_two(lines: &str) -> isize {
    let lines: Vec<&str> = lines.lines().collect();
    let mut o_lines = lines.clone();
    let mut c_lines = lines.clone();
    let mut oxygen = String::new();
    let mut carbon = String::new();

    // find oxygen generator rating
    for i in 0..lines.len() {
        if o_lines.len() == 1 {
            oxygen.push_str(o_lines[0]);
            break;
        }

        let mut count = (0, 0);
        let mut kill;
        for line in &o_lines {
            match line.as_bytes()[i] {
                48 => { count.0 += 1 }
                _ => { count.1 += 1 }
            }
        }

        // find the greater value
        if count.0 > count.1 {
            kill = 48;
        } else {
            kill = 49;
        }
        
        o_lines.retain(|x| x.as_bytes()[i] == kill);
    }

    // find CO2 scrubber rating
    for i in 0..lines.len() {
        if c_lines.len() == 1 {
            carbon.push_str(c_lines[0]);
            break;
        }

        let mut count = (0, 0);
        let mut kill;
        for line in &c_lines {
            match line.as_bytes()[i] {
                48 => { count.0 += 1 }
                _ => { count.1 += 1 }
            }
        }
        
        // find the greater value
        if count.1 < count.0 {
            kill = 49;
        } else {
            kill = 48;
        }
        
        c_lines.retain(|x| x.as_bytes()[i] == kill);
    }

    isize::from_str_radix(&oxygen, 2).unwrap() * isize::from_str_radix(&carbon, 2).unwrap()
}

