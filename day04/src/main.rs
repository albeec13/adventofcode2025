
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, Rgb};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
struct RollMatrix {
    rows: Vec<Vec<char>>,
    roll_indicator: char,

    // added for animation generation
    frames: Vec<Vec<Vec<char>>>,
}

impl RollMatrix {
    fn new(roll_indicator: char) -> Self {
        Self {rows: Vec::new(), roll_indicator, frames: Vec::new()}
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
        let mut weights = self.rows.clone();       
        let mut stuck_cnt = 0;

        loop  {
            self.frames.push(self.rows.clone());

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

                        weights[j][i] = near.to_string().chars().nth(0).unwrap_or('0');
                        //print!("{}",  roll);
                    }
                }
                //println!();
            }
            println!();
        
            for row in weights.iter_mut() {
                for col in row.iter_mut() {
                    let new_cell = match *col {
                        c if c.is_digit(10) => {
                            if c.to_digit(10).unwrap_or(u32::MAX) < 4 {
                                new_cnt += 1;
                                'X'
                            } else {
                                '@'
                            }
                        }
                        'X' => 'x',
                        'x' => 'Y',
                        'Y' => 'y',
                        'y' => 'Z',
                        'Z' => 'z',
                        'z' => 'A',
                        'A' => 'a',
                        'a' => '.',
                        other => other,
                    };
                    *col = new_cell;
                    print!("{}", col);
                }
                println!();
            }
            
            cnt += new_cnt;
            println!("Rolls removed: {}", new_cnt);

            if new_cnt == 0 {
                stuck_cnt += 1;
            } else {
                stuck_cnt = 0;
            }

            // Only run once for original day 1 puzzle, otherwise continue to modify matrix (but run 8 extra times for animation color transitions)
            if single || stuck_cnt == 8 {
                break;
            }

            self.rows = weights.clone();
        }

        //self.save_image("test.png", 1000);
        if !single {
            self.frames.push(self.rows.clone());
            println!("Generating animation using {} frames...", self.frames.len());
            self.save_gif("animation.gif", 1000, 75);
        }

        cnt
    }

    // Image generating code - assisted by copilot initially
    /// Convert the grid into a scaled image using pixel duplication.
    fn scaled_image_from_grid(
        grid: &[Vec<char>],
        target_size: u32
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> 
    {
        let height = grid.len() as u32;
        let width = grid[0].len() as u32;

        let scale = target_size / width.max(height);
        let scaled_w = width * scale;
        let scaled_h = height * scale;

        let mut img = ImageBuffer::new(scaled_w, scaled_h);

        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let color = match cell {
                    '@' => Rgb([200, 200, 200]),
                    'X' => Rgb([255, 242, 116]),
                    'x' => Rgb([255, 199, 92]),
                    'Y' => Rgb([255, 153, 67]),
                    'y' => Rgb([255, 102, 41]),
                    'Z' => Rgb([255, 0, 0]),
                    'z' => Rgb([174, 0, 0]),
                    'A' => Rgb([99, 0, 0]),
                    'a' => Rgb([34, 0, 0]),
                    '.' => Rgb([0, 0, 0]),
                    _   => Rgb([128, 128, 128]),
                };

                for dy in 0..scale {
                    for dx in 0..scale {
                        img.put_pixel(
                            (x as u32 * scale) + dx,
                            (y as u32 * scale) + dy,
                            color,
                        );
                    }
                }
            }
        }

        img
    }

    /// Save the current grid as a scaled PNG.
    #[allow(dead_code)]
    fn save_image(&self, path: &str, target_size: u32) {
        let img = Self::scaled_image_from_grid(&self.rows, target_size);
        img.save(path).unwrap();
    }

    /// Save a sequence of RollMatrix frames as a GIF (no temp files).
    fn save_gif(&self, path: &str, target_size: u32, delay_ms: u16) {
        // All frames must be the same size
        let base_w = self.frames[0][0].len() as u32;
        let base_h = self.frames[0].len() as u32;

        let scale = target_size / base_w.max(base_h);
        let gif_w = base_w * scale;
        let gif_h = base_h * scale;

        let mut file = std::fs::File::create(path).unwrap();
        let mut encoder = Encoder::new(&mut file, gif_w as u16, gif_h as u16, &[]).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();

        for (idx, frame) in self.frames.iter().enumerate() {
            let img = Self::scaled_image_from_grid(frame, target_size);

            // Convert to raw RGB for GIF
            let mut rgb_data = Vec::with_capacity((gif_w * gif_h * 3) as usize);
            for pixel in img.pixels() {
                rgb_data.extend_from_slice(&pixel.0);
            }

            let mut gif_frame = Frame::from_rgb(gif_w as u16, gif_h as u16, &rgb_data);
            
            // GIF uses 1/100s units
            let delay = match idx {
                0 => 200,
                n if n == self.frames.len() - 1 => 200,
                _ => delay_ms / 10,
            };
            gif_frame.delay = delay;

            encoder.write_frame(&gif_frame).unwrap();
        }
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
