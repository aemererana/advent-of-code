use regex::Regex;
use std::{fs, vec};



fn main() {
    let input_res = fs::read_to_string("input.txt");
    let mut vec_valid_part_num: Vec<i32> = vec![];

    match input_res {
        Err(_) => {
            println!("Error: Failed to open the file!");
        },


        Ok(content) => {
            let content_line_split: Vec<&str> = content.lines().collect();
            let total_line_count = content_line_split.len();
            println!("Content Info - Lines {}, Size: {}", total_line_count, content.len());

            // const SYMBOLS_LIST: &str = r"\[\]!@#$%^&*(),|\\+-/\?";
            const SYMBOLS_LIST: &str = r"^a-zA-Z0-9\.";

            let re_digits = Regex::new(format!(r"(?<number>\d+)").as_str()).unwrap();
            // let re_digits = Regex::new(r"(?<number>\d+)").unwrap();
            let re_symbols = Regex::new(format!(r"(?<symbol>[{SYMBOLS_LIST}])").as_str()).unwrap();

            for (i, line) in content_line_split.iter().enumerate() {
                // check current line numbv if adjacent to symbols
                println!(" Line {}: ", i + 1);

                // get number per line and check if adjacent to a symbol
                for capture in re_digits.captures_iter(line) {
                    let digit = capture.name("number").unwrap();

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

            println!("TOTAL: {}", total_sum);

        } // end ok(T) match
    } // end match



}