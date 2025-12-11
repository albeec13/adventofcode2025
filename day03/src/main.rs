
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

fn get_joltage_day1(bank: &str) -> u64 {
    get_joltage_num_digits(bank, 2)
}

fn get_joltage_day2(bank: &str) -> u64 {
    get_joltage_num_digits(bank, 12)
}

fn get_joltage_num_digits(bank: &str, num_digits: usize) -> u64 {
    let mut joltage_vec: Vec<char> = [].to_vec();
    let mut next_start_index: usize = 0;

    for digit in (0..=(num_digits.saturating_sub(1))).rev() {
        let (pos, val) = get_joltage_single_high(&bank.char_slice(next_start_index, bank.chars().count().saturating_sub(digit)));
        next_start_index += pos.saturating_add(1); //each call moves start index to 0 since it's a sub-slice so add this to previous starting location
        joltage_vec.push(val);
    }

    joltage_vec.iter().collect::<String>().parse::<u64>().unwrap_or(0)
}

fn get_joltage_single_high(bank: &str) -> (usize, char) {
    bank.chars()
        .enumerate()
        .scan((usize::MIN, '0'), |best, (i, c)| {
            if best.1 == '9' {
                return None;
            }

            if c.is_ascii_digit() && c > best.1 {
                *best = (i, c);
            }

            Some(*best)
        })
        .last()
        .unwrap_or((usize::MIN, '0'))
}

fn main() {
    let mut total_joltage_day1: u64 = 0;
    let mut total_joltage_day2: u64 = 0;

    if let Ok(lines) = read_lines("./input") {
        for bank in lines.map_while(Result::ok) {            
            print!("Bank: {}", bank);
            let bank_joltage_day1 = get_joltage_day1(&bank);
            let bank_joltage_day2 = get_joltage_day2(&bank);
            total_joltage_day1 += bank_joltage_day1;
            total_joltage_day2 += bank_joltage_day2;
            println!(" Jolts (part 1, part 2): {}, {}",bank_joltage_day1, bank_joltage_day2);
        }
    }
    println!("Total Joltage (part 1, part 2): {}, {}", total_joltage_day1, total_joltage_day2);
}
