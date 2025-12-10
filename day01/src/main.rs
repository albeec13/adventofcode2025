
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Dial struct represents a rotary dial mechanism.
struct Dial {
    upper_limit: u32,
    position: u32,
}

// Initialze a new Dial with a specified upper limit.
impl Dial {
    fn new(upper_limit: u32) -> Self {
        Dial {
            upper_limit,
            position: 50,
        }
    }

    // Private method to rotated the dial left or right, wrapping around the limits
    fn rotate(&mut self, direction: char, amount: &str) -> Result<u8, Box<dyn std::error::Error>> {

        let mut uamount = amount.parse::<u32>()?;
        let mut zero_count: u8 = 0;

        if uamount > (self.upper_limit + 1) {
            zero_count += (uamount / (self.upper_limit + 1)) as u8;
        }
        uamount = uamount % (self.upper_limit + 1);
        
        match direction {
            'L' => {
                if uamount > self.position {
                    if self.position != 0 {
                        zero_count += 1;
                    }
                    self.position = self.position + self.upper_limit + 1 - uamount;
                } else {
                    self.position -= uamount;
                }
            }
            'R' => {
                self.position = self.position + uamount;
                if self.position > self.upper_limit + 1 {
                    zero_count += 1;
                }
                self.position = self.position % (self.upper_limit + 1);
            }
            _ => {
                return Err(String::from("Invalid input direction").into());
            }
        }
        if self.position == 0 {zero_count += 1};
        println!("Dial rotated {}{} to position {}, counted {} zeroes", direction, amount, self.position, zero_count);
        Ok(zero_count)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut dial = Dial::new(99);
    let mut code: u32 = 0;

    if let Ok(lines) = read_lines("./input") {
        for line in lines.map_while(Result::ok) {
            if let Ok(result) = dial.rotate(line.chars().next().unwrap(), &line[1..]) {
                code += result as u32;
            }
        }
    }

    println!("Code is: {}", code);
}
