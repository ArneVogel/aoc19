use math::round;
use std::env;
use std::io::{self, BufRead};

fn solve_one(input: Vec<String>) {
    let mut sum = 0;
    for n in input {
        let number = n.parse::<i32>().unwrap();
        sum += round::floor((number / 3).into(), 1) as i32 - 2;
    }
    println!("{}", sum);
}
fn solve_two(input: Vec<String>) {
    let mut sum: i64 = 0;
    let mut tmpsum: i64 = 0;
    let mut tmp = 0;
    for n in input {
        let number = n.parse::<i64>().unwrap();
        tmp = round::floor((number / 3) as f64, 1) as i64 - 2;
        tmpsum += tmp;
        while tmp > 0 {
            tmp = round::floor((tmp / 3) as f64, 1) as i64 - 2;
            if (tmp > 0) {
                tmpsum += tmp;
            }
        }
        sum += tmpsum;
        tmpsum = 0;
    }
    println!("{}", sum);
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
