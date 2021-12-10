use std::error::Error;
use std::fs;

const DEFAULT_TIMER: i32 = 6;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("../input.txt")?;
    println!("Part one solution: {}", part_one(&data));
    println!("Part two solution: {}", part_two(&data));
    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct Fish {
    pub d_timer: i32,
    pub i_timer: i32,
}

struct FishList {
    pub list: Vec<Fish>,
}

impl FishList {
    pub fn new(lines: &str) -> FishList {
        let mut list: Vec<Fish> = Vec::new();
        for line in lines.lines() {
            let mut int_lines: Vec<Fish> = line
                .split(",")
                .map(|item| Fish { d_timer: DEFAULT_TIMER, i_timer: item.parse().unwrap() })
                .collect();
            list.append(&mut int_lines);
        }

        FishList { list }
    }

    fn pass_day(&mut self) {
        for i in 0..self.list.len() {
            if self.list[i].i_timer == 0 {
                self.list.push(Fish { d_timer: DEFAULT_TIMER, i_timer: self.list[i].d_timer + 2 });
                self.list[i].i_timer = self.list[i].d_timer;
            } else {
                self.list[i].i_timer -= 1;
            }
        }
    }

    pub fn pass_days(&mut self, days: i32) {
        if days >= 0 {
            for i in 0..days {
                self.pass_day();
            }
        }
    }
}

fn part_one(lines: &str) -> i32 {
    let mut fish_list = FishList::new(lines);
    fish_list.pass_days(80);
    fish_list.list.len().try_into().unwrap()
}

fn part_two(lines: &str) -> i32 {
    0
}
