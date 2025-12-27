
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_fresh_ids_part1(ranges: Vec<(u64, u64)>, ids: Vec<u64>) -> u64 {
    let mut total = 0;
    let mut id_found: Vec<bool> = Vec::new();

    id_found.resize(ids.len(), false);

    for (start, end) in ranges {
        for (idx, id) in ids.iter().enumerate() {
            if !id_found[idx] && (start..=end).contains(id) {
                total += 1;
                id_found[idx] = true;
                //println!("Found fresh id {} (index {}) from range {}..{}", ids[idx], idx, start, end);
            }
        }
    }

    total
}

fn count_fresh_ids_part2(mut ranges: Vec<(u64, u64)>) -> u64 {
    let mut total: u64 = 0;

    ranges.sort();

    for i in 0..ranges.len() - 1 {
        let (left, right) = ranges.split_at_mut(i + 1);
        let curr = &mut left[i];
        let next = &mut right[0];

        // if upper end of current range is equal or greater than lower end of next range, move lower end of next range to upper end of current range + 1
        if curr.1 >= next.0 {
            //println!("curr.1 ({}) >= next.0 ({})", curr.1, next.0);
            next.0 = curr.1 + 1;

            // after that move, if the next range now has an inverted range (lower is higher than upper), it's no longer valid, so copy the current
            // upper range marker to the next upper range marker instead, to continue propagating it to the next range
            if next.0 > next.1 {
                //println!("next.0 ({}) > next.1 ({})", next.0, next.1);
                next.1 = curr.1;
            }
        }

        /* For example, if we have 3 ranges in a row like:
                1) 12345-12355
                2) 12348-12354
                3) 12352-12359
           When comparing the first 2 rows, the upper range on the 1st row exceeds to the lower range on the 2nd row, so the 2nd row becomes as follows
           when changing its lower range to current upper range + 1 (12356):
                2) 12356-12354
           This leaves the 2nd row with an inverted range, so we change the upper part of the range to the current upper range of 12355, which will still be
           lower than the lower part of the range:
                2) 12356-12355
           This propagates the upper range from row 1 to the next comparison, while still leaving row 2 inverted (total count will be 0 for this row). Now,
           comparing the new row 2 to row 3:
                2) 12356-12355
                3) 12352-12359
           Once again, the upper range of row 2 is higher than the lower range of row 3, so we change the lower range of row 3 to be 12355 + 1 like so:
                3) 12356-12359
           This time, it's a valid range, with a new upper limit of 12359 to be compared to row 4, etc.
         */
    }

    // count up the lengths of each adjusted range now. ranges with inverted values (higher..lower) will count for 0 automatically
    for range in ranges {
        //println!("{:?}.count() = {}", range, (range.0..=range.1).count());
        total += (range.0..=range.1).count() as u64;
    }

    total
}

fn main() {
    let mut id_ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut end_of_ranges: bool = false;

    // insert rows from input, when we reach 3 total, begin checking for movable rolls at position 2 in the 3-row moving window
    if let Ok(lines) = read_lines("./input") {
        for row in lines.map_while(Result::ok) {
            if !end_of_ranges {
                if row.is_empty() {
                    end_of_ranges = true;
                } else {
                    let begin_end: Vec<_> = row.split('-')
                                       .map(|x| x.parse::<u64>().unwrap_or_else(|_x| 0))
                                       .collect();

                    id_ranges.push((begin_end[0], begin_end[1]));
                }
            } else {
                ids.push(row.parse::<u64>().unwrap_or_else(|_x| 0));
            }
            match row.as_str() {
                 "" => println!("end of ranges"),
                _ => println!("{}", row)
            };
        }
    }

    println!("Part 1 count of fresh ids: {}", count_fresh_ids_part1(id_ranges.clone(), ids));
    println!("Part 2 count of fresh ids: {}", count_fresh_ids_part2(id_ranges));

}
