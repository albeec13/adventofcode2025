
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

#[derive(Clone)]
struct RollMatrix {
    rows: Vec<Vec<char>>,
    roll_indicator: char,
}

impl RollMatrix {
    fn new(roll_indicator: char) -> Self {
        Self {rows: Vec::new(), roll_indicator}
    }

    fn push(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }

    fn check_at(&self, row_index: usize, row_offset: isize, col_index: usize, col_offset: isize) -> u64 {
        row_index.checked_add_signed(row_offset)
            .and_then(|r| self.rows.get(r))
            .and_then(|row| {
                col_index.checked_add_signed(col_offset)
                .and_then(|c| row.get(c))
            })
            .map(|elem| (*elem == self.roll_indicator) as u64)
            .unwrap_or(0)
    }

    fn count_movable_rolls(&mut self, single: bool) -> u64 {
        let mut cnt: u64 = 0;
        let mut weights = self.clone();

        
        loop  {
            let mut new_cnt = 0;
            for (j, row) in self.rows.iter().enumerate() {
                for (i, roll) in row.iter().enumerate() {
                    if roll != &self.roll_indicator {
                        //print!("{}", roll);
                        continue;
                    } else {
                        let mut near = 0;
                        
                        near += self.check_at(j, -1, i, -1);    //check top-left
                        near += self.check_at(j, -1, i, 0);     //check above
                        near += self.check_at(j, -1, i, 1);     //check top-right                    
                        near += self.check_at(j, 0, i, -1);     //check left
                        near += self.check_at(j, 0, i, 1);      //check right
                        near += self.check_at(j, 1, i, -1);     //check bottom-left
                        near += self.check_at(j, 1, i, 0);      //check bottom
                        near += self.check_at(j, 1, i, 1);      //check bottom-right

                        weights.rows[j][i] = near.to_string().chars().nth(0).unwrap_or('0');
                        //print!("{}",  roll);
                    }
                }
                //println!();
            }
            println!();
        
            for row in weights.rows.iter_mut() {
                for col in row.iter_mut() {
                    if col.is_numeric() {
                        if col.to_digit(10).unwrap_or(u32::MAX) < 4 {
                            *col = 'X';
                            print!("{}", col);
                            *col = '.';
                            new_cnt += 1;
                        } else {
                            *col = '@';
                            print!("{}", col);
                        }
                    } else {
                        print!("{}", col);
                    }
                }
                println!();
            }
            
            cnt += new_cnt;
            println!("Rolls removed: {}", new_cnt);

            // Only run once for original day 1 puzzle, otherwise continue to modify matrix
            if single || new_cnt == 0 {
                break;
            }

            *self = weights.clone();
        }

        cnt
    }
}

fn main() {
    //process 3 rows at a time in a moving window, where initally the first row is null, and at the end the last row is null
    let mut rm: RollMatrix = RollMatrix::new('@');
    let mut total_movable_rolls_part1 = 0;
    let mut total_movable_rolls_part2 = 0;
    
    // insert rows from input, when we reach 3 total, begin checking for movable rolls at position 2 in the 3-row moving window
    if let Ok(lines) = read_lines("./input") {
        for row in lines.map_while(Result::ok) {            
            rm.push(row.chars().collect());
        }
    }

    total_movable_rolls_part1 += rm.count_movable_rolls(true);
    println!("Total movable rolls: {}", total_movable_rolls_part1);
    
    total_movable_rolls_part2 += rm.count_movable_rolls(false);
    println!("Total movable rolls: {}", total_movable_rolls_part2);
}
