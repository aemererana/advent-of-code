use regex::{Regex, Match};
use core::num;
use std::{fs, vec};

fn part1(content: &str) {
    let mut vec_valid_part_num: Vec<i32> = vec![];

    let content_line_split: Vec<&str> = content.lines().collect();
    let total_line_count = content_line_split.len();
    println!("Content Info - Lines {}, Size: {}", total_line_count, content.len());

    // const SYMBOLS_LIST: &str = r"\[\]!@#$%^&*(),|\\+-/\?";
    const SYMBOLS_LIST: &str = r"^a-zA-Z0-9\.";

    let re_digits = Regex::new(format!(r"(\d+)").as_str()).unwrap();
    // let re_digits = Regex::new(r"(?<number>\d+)").unwrap();
    let re_symbols = Regex::new(format!(r"([{SYMBOLS_LIST}])").as_str()).unwrap();

    for (i, line) in content_line_split.iter().enumerate() {
        // check current line numbv if adjacent to symbols
        println!(" Line {}: ", i + 1);

        // get number per line and check if adjacent to a symbol
        for capture in re_digits.captures_iter(line) {
            let digit = capture.get(1).unwrap();

            // line info
            print!("\tnumber: {} - start: {} end: {}", digit.as_str(), digit.start(), digit.end());

            let start_substr = if digit.start() > 0 { digit.start() - 1 } else { 0 };
            let end_substr = if digit.end() + 1 >= line.len() { line.len() } else { digit.end() + 1 };

            // check current line
            let number_padded = &line[start_substr..end_substr];
            if re_symbols.find(number_padded).is_some() {
                print!(" - VALID\n");
                vec_valid_part_num.push(digit.as_str().parse().unwrap());
                continue;
            }

            // check line before
            if i > 0 {
                let substr_prev_line = content_line_split[i-1];

                if re_symbols.find(&substr_prev_line[start_substr..end_substr]).is_some() {
                    print!(" - VALID\n");
                    vec_valid_part_num.push(digit.as_str().parse().unwrap());
                    continue;
                }
            }

            // check line after
            if i + 1 < content_line_split.len() {
                let substr_line_after = content_line_split[i+1];

                if re_symbols.find(&substr_line_after[start_substr..end_substr]).is_some() {
                    print!(" - VALID\n");
                    vec_valid_part_num.push(digit.as_str().parse().unwrap());
                    continue;
                }
            }

            println!("");
        } // end number parsing

    } // end line parsing

    // Total
    let mut total_sum = 0;
    vec_valid_part_num.iter().for_each(|val| total_sum += val);

    println!("Part 1 - TOTAL: {}", total_sum);
}


fn part2(content: &str) {
    println!("------------------ Part 2 ------------------");
    let re_digit = Regex::new(r"(\d+)").unwrap();
    let re_gear = Regex::new(r"(\*)").unwrap();

    let content_line_split: Vec<&str> = content.lines().collect();

    let mut gear_coords: Vec<Vec<i32>> = vec![];

    let get_full_num = |line: &str, gear_idx_start: usize, gear_idx_end: usize, start_at: usize| -> Option<(String, usize)> {
        for number_capture in re_digit.captures_iter(line) {
            let number = number_capture.get(1).unwrap();

            if start_at > number.start() {
                continue;
            }

            if number.start() >= gear_idx_start && number.start() <= gear_idx_end || 
                number.end() >= gear_idx_start && number.end() <= gear_idx_end ||
                gear_idx_start >= number.start() && gear_idx_start <= number.end() ||
                gear_idx_end >= number.end() && gear_idx_end <= number.end() {
                    return Some((number.as_str().to_string(), number.end()));
                }
        }
        None
    };

    // find number
    for (i, line) in content_line_split.iter().enumerate() {

        println!("Line {}", i + 1);

        for (j, gear_capture) in re_gear.captures_iter(line).enumerate() {
            let gear = gear_capture.get(1).unwrap();

            println!("\tGear {} on {}, {}", j, gear.start(), gear.end());

            let mut gear_ratio_nums: Vec<i32> = vec![];

            // let start_substr = if gear.start() > 0 { gear.start() - 1 } else { 0 };
            // let end_substr = if gear.end() + 1 >= line.len() { line.len() } else { gear.end() + 1 };

            let start_substr = gear.start();
            let end_substr = gear.end();

            if let Some((number, endpos)) = get_full_num(line, start_substr, end_substr, 0) {
                // number touching the gear, push to the array
                gear_ratio_nums.push(number.as_str().parse().unwrap());
                if let Some((another_num, _)) = get_full_num(line, start_substr, end_substr, endpos) {
                    gear_ratio_nums.push(another_num.as_str().parse().unwrap());
                }

                if gear_ratio_nums.len() == 2 {
                    gear_coords.push(gear_ratio_nums);
                    continue;
                }
            }

            // check line before
            if i > 0 {
                let prev_line = content_line_split[i-1];
                if let Some((number, endpos)) = get_full_num(prev_line, start_substr, end_substr, 0) {
                    // number touching the gear, push to the array
                    gear_ratio_nums.push(number.as_str().parse().unwrap());

                    if gear_ratio_nums.len() == 2 {
                        gear_coords.push(gear_ratio_nums);
                        continue;
                    }

                    if let Some((another_num, _)) = get_full_num(prev_line, start_substr, end_substr, endpos) {
                        gear_ratio_nums.push(another_num.as_str().parse().unwrap());
                    }
    
                    if gear_ratio_nums.len() == 2 {
                        gear_coords.push(gear_ratio_nums);
                        continue;
                    }
                }
            }

            // check line after
            if i + 1 < content_line_split.len() {
                let substr_line_after = content_line_split[i+1];
                if let Some((number, endpos)) = get_full_num(substr_line_after, start_substr, end_substr, 0) {
                    // number touching the gear, push to the array
                    gear_ratio_nums.push(number.as_str().parse().unwrap());

                    if gear_ratio_nums.len() == 2 {
                        gear_coords.push(gear_ratio_nums);
                        continue;
                    }

                    if let Some((another_num, _)) = get_full_num(substr_line_after, start_substr, end_substr, endpos) {
                        gear_ratio_nums.push(another_num.as_str().parse().unwrap());
                    }
    
                    if gear_ratio_nums.len() == 2 {
                        gear_coords.push(gear_ratio_nums);
                        continue;
                    }
                }
            }

            println!("Loner number found! {:?}", gear_ratio_nums)
        }
    }

    let mut total = 0;
    gear_coords.iter().for_each(|pair| total += pair[0] * pair[1]);
    println!("TOTAL: {}", total);
    
    println!("Debug: {:?}", gear_coords);

}

fn main() {
    let input_res = fs::read_to_string("input.txt");

    match input_res {
        Err(_) => {
            println!("Error: Failed to open the file!");
        },


        Ok(content) => {
            part1(&content);
            part2(&content);
        } // end ok(T) match
    } // end match



}