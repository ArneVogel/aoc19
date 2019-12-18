use std::env;
use std::io::{self, BufRead};

fn valid_one(input: i64) -> bool {
    let mut increasing = true;
    let mut last_num = 0;
    let mut double = false;
    for i in input.to_string().chars().map(|x| x.to_digit(10).unwrap()) {
        if i == last_num {
            double = true;
        }
        if i < last_num {
            increasing = false;
            break;
        }
        last_num = i;
    }
    return increasing && double;
}

fn valid_two(input: i64) -> bool {
    let mut increasing = true;

    let mut last_num = 0;
    let mut num_before_that = 1;

    let mut double = false;
    let string = input.to_string();
    let digits: Vec<u32> = string.chars().map(|x| x.to_digit(10).unwrap()).collect();
    for (pos, i) in digits.iter().enumerate() {
        if i == &last_num
            && ((pos == 5 && digits[3] != *i)
                || (pos <= 4 && pos > 1 && digits[pos + 1] != *i && digits[pos - 2] != *i)
                || (pos == 1 && digits[2] != *i))
        {
            //dbg!(digits.clone());
            double = true;
        }
        if i < &last_num {
            increasing = false;
            break;
        }
        last_num = *i;
    }
    return increasing && double;
}

fn solve_one(input: Vec<String>) {
    let range: Vec<i64> = input[0]
        .split("-")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut sum = 0;
    for i in range[0]..range[1] {
        if valid_one(i) {
            sum += 1;
        }
    }
    dbg!(sum);
}
fn solve_two(input: Vec<String>) {
    let range: Vec<i64> = input[0]
        .split("-")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut sum = 0;
    for i in range[0]..range[1] {
        if valid_two(i) {
            sum += 1;
        }
    }
    dbg!(sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        if line.chars().count() == 0 {
            break;
        }
        input.push(line);
        //println!("{}", line);
    }
    if &args[1] == "1" {
        solve_one(input);
    } else if &args[1] == "2" {
        solve_two(input);
    }
}
