use regex::{Regex, Captures};
use std::{fs, collections::HashMap};

// static 

fn parse_card_line(digit_re: &Regex, capture: &Captures) -> Vec<i32> {
    let mut win_numbers: Vec<i32> = vec![];
    for win_number in digit_re.captures_iter(capture.name("win_num").unwrap().as_str()) {
        win_numbers.push(win_number.get(1).unwrap().as_str().parse().unwrap());
    }

    let mut winning_combo: Vec<i32> = vec![];
    for captured_number in digit_re.captures_iter(capture.name("card_num").unwrap().as_str()) {

        let i32_captured_number = captured_number
                                        .get(1)
                                        .unwrap()
                                        .as_str()
                                        .parse::<i32>()
                                        .unwrap();

        if let Some(match_number) = 
            win_numbers
                .iter()
                .find(|&&x| 
                    i32_captured_number == x
                ) 
        {
            winning_combo.push(*match_number);
        }
    }

    // win
    // let aggregate = digit_re
    //     .captures_iter(capture.name("card_num").unwrap().as_str())
    //     .filter_map(|capture| {
    //         let captured_number = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
    //         match win_numbers.iter().find(|&&x| captured_number == x) {
    //             None => None,
    //             Some(match_number) => Some(match_number)
    //         }
    //     })
    //     .fold(1, |acc, _| acc + i32::pow(acc, 2));

    println!("DEBUG line num: {:?} - Win: {:?} - size: {}", capture.name("line_num").unwrap().as_str(), winning_combo, winning_combo.len());
    
    return winning_combo;
    
    // println!("DEBUG card_num: {:?}", capture.name("card_num"));
}

fn process_lines(content_in_lines_vec: &Vec<&str>) {
    let re_line = Regex::new(r"^Card\s+(?<line_num>\d+):\s+(?<win_num>.*)\s+\|\s+(?<card_num>.*)$").unwrap();
    let re_digit = Regex::new(r"(?<number>\d+)").unwrap();

    let mut total_sum = 0;
    let mut cards_token: HashMap<usize, i32> = HashMap::new();

    // init tokens
    for i in 0..content_in_lines_vec.len() {
        cards_token.insert(i, 1);
    }

    // iterate lines
    for (i, line) in content_in_lines_vec.iter().enumerate() {

        // check line is valid
        match re_line.captures(line) {
            // error
            None => {
                println!("There was an error parsing line {}", i);
            },

            // line is valid
            Some(capture) => {

                // get win combo
                let winning_combo = parse_card_line(&re_digit, &capture);

                // add tokens                
                if winning_combo.len() > 0 {
                    let start_idx = i + 1;
                    let winning_card_current_count: i32 = *cards_token.get(&i).unwrap_or(&0);
                    for j in start_idx..(start_idx + winning_combo.len()) {
                        let current_token_count = *cards_token.get(&j).unwrap_or(&0);
                        cards_token.insert(j, current_token_count + winning_card_current_count);
                    }
                }

                // count total
                total_sum += winning_combo.iter().fold(0, |acc, _| {
                    if acc == 0 { 1 } else { acc + acc }
                });
                
            },
        };
    }

    println!("Total: {}", total_sum);
    
    let total_count_of_card = cards_token.iter().fold(0, |acc, (_, count)| acc + count);
    println!("Total count of cards: {}", total_count_of_card);
    println!("HashMap {:?}", cards_token);
}

fn main() {
    let input_res = fs::read_to_string("input.txt");
    match input_res {
        Err(_) => {
            println!("There was an error opening the file!");
            return;
        },
        Ok(content) => {
            // parse line
            process_lines(&content.lines().collect());
        }
    }
}