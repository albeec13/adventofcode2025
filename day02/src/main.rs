
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_digit_count(value: &u64) -> u8 {
    match value {
        0..10 => 1,
        10..100 => 2,
        100..1000 => 3,
        1000..10000 => 4,
        10000..100000 => 5,
        100000..1000000 => 6,
        1000000..10000000 => 7,
        10000000..100000000 => 8,
        100000000..1000000000 => 9,
        1000000000..10000000000 => 10,
        10000000000..100000000000 => 11,
        100000000000..1000000000000 => 12,
        1000000000000..10000000000000 => 13,
        10000000000000..100000000000000 => 14,
        100000000000000..1000000000000000 => 15,
        1000000000000000..10000000000000000 => 16,
        10000000000000000..100000000000000000 => 17,
        100000000000000000..1000000000000000000 => 18,
        1000000000000000000..10000000000000000000 => 19,
        _ => 0,
    }
}


// Exploit the fact that numerical patterns like AA, ABAB, ABCABC, etc. can be created by multipling the base value (A, AB, ABC, etc.) by
// 10^(base digit count/2) + 1, i.e. 11, 101, 1001, etc. Obviously, repeating patterns can only happen for even digit counts, so discard any ranges
// that don't have even digit counts. Finally, for input ranges that change from odd to even or even to odd digit counts, we ignore odd inputs.
fn validate_ids_day1(min: &u64, max: &u64) -> u64 {
    let min_dig_cnt = get_digit_count(min);
    let max_dig_cnt = get_digit_count(max);
    let need_digit_cnt: bool = !(min_dig_cnt == max_dig_cnt && max_dig_cnt % 2 == 1);

    // ignore invalid inputs
    if min_dig_cnt == 0 || max_dig_cnt == 0 {
        return 0;
    }

    print!("[{}-{}] invalids: ", min, max);

    let result = (*min..=*max).fold(0, |acc: u64, val: u64| {
        let dig_cnt = if need_digit_cnt {get_digit_count(&val)} else {min_dig_cnt};
        let mut accum: u64 = acc;

        if dig_cnt % 2 == 0 && val % ((10_u64.pow((dig_cnt/2).into())) + 1) == 0 {
            print!("{} ", val);
            accum += val;
        }
        accum
    });

    result //default
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        for line in lines.map_while(Result::ok) {
            let mut sum: u64 = 0;
            
            for id_range in line.split(',') {
                let mut id_min_max = id_range.split('-');

                if let (Some(id_min), Some(id_max)) = (id_min_max.next(), id_min_max.next()) {
                    if let (Ok(min), Ok(max)) = (id_min.parse::<u64>(), id_max.parse::<u64>()) {
                        sum += validate_ids_day1(&min, &max);
                        println!();

                    } else {
                        println!("Could not parse min and max values into integers!");
                    }

                } else {
                    println!("Got malformed input range!");
                }
            }
            println!("Sum of invalids: {}", sum);
        }
    }
}
