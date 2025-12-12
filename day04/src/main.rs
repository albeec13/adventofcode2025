
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub trait CharSlice {
    fn char_slice(&self, start: usize, end: usize) -> &str;
}

impl CharSlice for str {
    fn char_slice(&self, start: usize, end: usize) -> &str {
        let start_byte = self.char_indices().nth(start).map(|(i, _)| i).unwrap_or(self.len());
        let end_byte = self.char_indices().nth(end).map(|(i, _)| i).unwrap_or(self.len());
        &self[start_byte..end_byte]
    }
}

struct RollingMatrix<T> {
    rows: Vec<Vec<T>>,
    max_rows: usize,
    roll_indicator: T,
}

impl<T: std::fmt::Display + std::cmp::PartialEq> RollingMatrix<T> {
    fn new(roll_indicator: T) -> Self {
        Self {rows: Vec::new(), max_rows: 3, roll_indicator}
    }

    fn push(&mut self, row: Vec<T>) {
        if self.rows.len() == self.max_rows {
            self.rows.remove(0);
        }
        self.rows.push(row);
    }

    fn check_at(&self, row: usize, i: usize, offset: isize) -> u64 {
        i.checked_add_signed(offset)
            .and_then(|idx| self.rows[row].get(idx))
            .map(|c| (*c == self.roll_indicator) as u64)
            .unwrap_or(0)
    }

    fn count_movable_rolls(&self) -> u64 {
        let mut cnt: u64 = 0;
        if self.rows.len() < self.max_rows {
            return 0;
        } else {
            //check above and below 2nd row to see how many rolls surround each roll in that row and return total
            for (i, roll) in self.rows[1].iter().enumerate() {
                if roll != &self.roll_indicator {
                    print!("{}", roll);
                    continue;
                } else {
                    let mut near = 0;
                    if  !self.rows[0].is_empty() {
                        //check top-left
                        near += self.check_at(0, i, -1);

                        //check above
                        near += self.check_at(0, i, 0);

                        //check top-right
                        near += self.check_at(0, i, 1);
                    }

                    //check left
                    near += self.check_at(1, i, -1);

                    //check right
                    near += self.check_at(1, i, 1);
                    
                    if !self.rows[2].is_empty() {
                        //check bottom-left
                        near += self.check_at(2, i, -1);

                        //check bottom
                        near += self.check_at(2, i, 0);

                        //check bottom-right
                        near += self.check_at(2, i, 1);
                    }

                    if near < 4 {
                        cnt += 1;
                    }
                    print!("{}", near);
                }
            }
        }
        println!();
        cnt
    }
}

fn main() {
    //process 3 rows at a time in a moving window, where initally the first row is null, and at the end the last row is null
    let mut rm: RollingMatrix<char> = RollingMatrix::new('@');
    let mut total_movable_rolls_part1 = 0;

    // insert empty first row
    rm.push(Vec::new());
    
    // insert rows from input, when we reach 3 total, begin checking for movable rolls at position 2 in the 3-row moving window
    if let Ok(lines) = read_lines("./input") {
        for row in lines.map_while(Result::ok) {            
            rm.push(row.chars().collect());
            //println!("{} matrix size {}", row, rm.rows.len());

            if rm.rows.len() == 3
            {
                total_movable_rolls_part1 += rm.count_movable_rolls();
            }
        }
    }

    // insert one more empty last row and check bottom row (before empty row) for movable rolls
    rm.push(Vec::new());
    total_movable_rolls_part1 += rm.count_movable_rolls();

    println!("Total movable rolls: {}", total_movable_rolls_part1);
}
