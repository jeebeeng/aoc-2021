use std::error::Error;
use std::fs;
use std::str;

const SIZE: usize = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("../input.txt")?;
    println!("Part one solution: {}", part_one(&data));
    println!("Part two solution: {}", part_two(&data));
    Ok(())
}

#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u32>>,
    overlap: u32,
}

impl Map {
    pub fn new() -> Map {
        let map = vec![vec![0; SIZE]; SIZE];
        Map { map, overlap: 0 }
    }

    pub fn get_points(line: &str) -> Vec<Point> {
        let points_str = line.split(" -> ");
        let mut points: Vec<Point> = Vec::new();

        for point_str in points_str {
            let values: Vec<&str> = point_str.split(",").collect();
            points.push(Point(values[0].parse().unwrap(), values[1].parse().unwrap()));
        }

        points
    }

    pub fn map_line_one(&mut self, a: &Point, b: &Point) {
        if a.0 == b.0 { // vertical line 
            let mut high = 0;
            let mut low = 0;

            if a.1 > b.1 { 
                low = b.1;
                high = a.1;
            } else { 
                low = a.1;
                high = b.1;
            };

            for y in low..=high {
                self.map[y as usize][a.0 as usize] += 1;
                if self.map[y as usize][a.0 as usize] == 2 {
                    self.overlap += 1;
                }
            }

        } else if a.1 == b.1 { // horizontal line
            let mut high = 0;
            let mut low = 0;

            if a.0 > b.0 { 
                low = b.0;
                high = a.0;
            } else { 
                low = a.0;
                high = b.0; 
            };

            for x in low..=high {
                self.map[a.1 as usize][x as usize] += 1;
                if self.map[a.1 as usize][x as usize] == 2 {
                    self.overlap += 1;
                }
            }

        }  
    }

    pub fn map_line_two(&mut self, a: &Point, b: &Point) {
        if a.0 == b.0 { // vertical line 
            let mut high = 0;
            let mut low = 0;

            if a.1 > b.1 { 
                low = b.1;
                high = a.1;
            } else { 
                low = a.1;
                high = b.1;
            };

            for y in low..=high {
                self.map[y as usize][a.0 as usize] += 1;
                if self.map[y as usize][a.0 as usize] == 2 {
                    self.overlap += 1;
                }
            }

        } else if a.1 == b.1 { // horizontal line
            let mut high = 0;
            let mut low = 0;

            if a.0 > b.0 { 
                low = b.0;
                high = a.0;
            } else { 
                low = a.0;
                high = b.0; 
            };

            for x in low..=high {
                self.map[a.1 as usize][x as usize] += 1;
                if self.map[a.1 as usize][x as usize] == 2 {
                    self.overlap += 1;
                }
            }

        } else { // diagonal line
            let mut xs = 1;
            let mut ys = 1;
            let mut diff = (a.0 - b.0).abs();

            if a.0 > b.0 {
                xs = -1;
            } 

            if a.1 > b.1 {
                ys = -1;
            } 

            for i in 0..=diff {
                self.map[(a.1 + (i * ys)) as usize][(a.0 + (i * xs)) as usize] += 1;
                if self.map[(a.1 + (i * ys)) as usize][(a.0 + (i * xs)) as usize] == 2 {
                    self.overlap += 1;
                }
            }
        } 
    }
}
    
fn part_one(lines: &str) -> u32 {
    let mut map = Map::new();
    
    lines.lines().for_each(|line| {
        let points = Map::get_points(line);
        map.map_line_one(&points[0], &points[1]);
    });

    map.overlap
}

fn part_two(lines: &str) -> u32 {
    let mut map = Map::new();
    
    lines.lines().for_each(|line| {
        let points = Map::get_points(line);
        map.map_line_two(&points[0], &points[1]);
    });

    map.overlap
}
