use std::error::Error;
use std::fs;
use std::str;

const SIZE: i8 = 5;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("/home/bang/projects/aoc-2021/day04/input.txt")?;
    println!("Part one solution: {}", part_one(&data));
    let result = part_two(&data);
    println!("Part two solution: {}", result);
    Ok(())
}

struct Bingo {
    pub drawn: Vec<u8>,
    pub boards: Vec<Vec<Vec<u8>>>,
    pub won_boards: Vec<Vec<Vec<u8>>>,
    pub curr_drawn: u8,
    pub last_drawn: u8,
}

impl Bingo {
    pub fn new(lines: &str) -> Bingo {
        // get the drawn numbers from input
        let drawn: Vec<u8> = lines
            .lines()
            .next()
            .unwrap()
            .split(",")
            .into_iter()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        // get each board into 2D vector and store into vector
        let mut lines: Vec<&str> = lines.lines().collect();
        lines.drain(0..2); // remove first two lines of input lines

        let mut boards: Vec<Vec<Vec<u8>>> = Vec::new();
        let mut board: Vec<Vec<u8>> = Vec::new();

        for line in lines {
            if line.is_empty() {
                boards.push(board);
                board = Vec::new();
            } else {
                let row: Vec<u8> = line
                    .split_whitespace()
                    .map(|x| {
                        return x.trim().parse::<u8>().unwrap();
                    })
                    .collect();
                board.push(row);
            }
        }
        boards.push(board);

        Bingo { drawn, boards, won_boards: Vec::new(), curr_drawn: 0, last_drawn: 0, }
    }

    pub fn check_win(board: &Vec<Vec<u8>>, drawn: &[u8]) -> bool {
        // check each row
        for row in board {
            if row.iter().all(|item| drawn.contains(item)) {
                return true;
            }
        }

        // check each column
        for i in 0..SIZE {
            let mut column: Vec<u8> = Vec::new();

            for n in 0..SIZE {
                column.push(board[n as usize][i as usize]);
            }

            if column.iter().all(|item| drawn.contains(item)) {
                return true;
            }
        }

        false
    }

    fn check_boards(&mut self) -> Vec<Vec<Vec<u8>>> {
        let mut winners: Vec<Vec<Vec<u8>>> = Vec::new();

        self.boards.retain(|board| {
            let mut keep = true;
            if Bingo::check_win(board, &self.drawn[0..self.curr_drawn as usize]) {
                self.won_boards.push(board.clone());
                winners.push(board.clone());
                self.last_drawn = self.curr_drawn;
                keep = false;
            }
            keep
        });
        winners
    }

    pub fn draw(&mut self) -> Vec<Vec<Vec<u8>>> {
        if self.curr_drawn as usize != self.drawn.len() {
            self.curr_drawn += 1;
            return self.check_boards();
        } 

        Vec::new()
    }
}

fn score(board: &Vec<Vec<u8>>, drawn: &[u8]) -> u32 {
    let mut score: u32 = 0;

    if !board.is_empty() {
        for row in board {
            for num in row {
                if !drawn.contains(num) {
                    score += *num as u32;
                }
            }
        }
    }

    score * drawn[drawn.len() - 1] as u32
}

fn part_one(lines: &str) -> u32 {
    let mut bingo = Bingo::new(lines);
    let mut winners: Vec<Vec<Vec<u8>>> = Vec::new();

    while winners.is_empty() {
        winners = bingo.draw();
    }
    let winners = winners.get(0).unwrap();
    score(&winners, &bingo.drawn[0..bingo.curr_drawn as usize])
}

fn part_two(lines: &str) -> u32 {
    let mut bingo = Bingo::new(lines);

    while !bingo.boards.is_empty() {
        bingo.draw();
    }
    let winner: &Vec<Vec<u8>> = bingo.won_boards.get(bingo.won_boards.len() - 1).unwrap();
    score(&winner, &bingo.drawn[0..bingo.last_drawn as usize])
}
