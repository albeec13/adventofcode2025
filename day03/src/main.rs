
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

fn get_joltage(bank: &str) -> u64 {
    let first_joltage = get_joltage_generic(&bank.char_slice(0, bank.chars().count().saturating_sub(1)));
    let second_joltage = get_joltage_generic(&bank.char_slice(first_joltage.0.saturating_add(1), bank.chars().count()));

    [first_joltage.1, second_joltage.1].iter().collect::<String>().parse::<u64>().unwrap_or(0)
}

fn get_joltage_generic(bank: &str) -> (usize, char) {
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
    let mut total_joltage: u64 = 0;
    if let Ok(lines) = read_lines("./input") {
        for bank in lines.map_while(Result::ok) {            
            print!("Bank: {}", bank);
            let bank_joltage = get_joltage(&bank);
            total_joltage += bank_joltage;
            println!(" Jolts: {}",bank_joltage);
        }
    }
    println!("Total Joltage: {}", total_joltage);
}
