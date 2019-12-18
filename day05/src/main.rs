use math::round;
use std::env;
use std::io::{self, BufRead};

fn parse_parameter_mode(pos: usize, numbers: Vec<i64>) -> (i64, i64, i64, i64) {
    let digits: Vec<u32> = numbers[pos]
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let operation: u32 = 10 * digits.get(digits.len() - 2).unwrap() + digits.last().unwrap();
    let mut operand1: i64 = 0;
    let mut operand2: i64 = 0;
    let mut target: i64 = 0;

    if digits.len() >= 3 && digits.get(digits.len() - 3).unwrap() == &1 {
        operand1 = (pos + 1) as i64;
    } else {
        operand1 = numbers[pos + 1];
    }

    if digits.len() >= 4 && digits.get(digits.len() - 4).unwrap() == &1 {
        operand2 = (pos + 2) as i64;
    } else {
        operand2 = numbers[pos + 2];
    }

    if digits.len() >= 5 && digits.get(digits.len() - 5).unwrap() == &1 {
        operand1 = (pos + 3) as i64;
    } else {
        target = numbers[pos + 3];
    }

    return (operation as i64, operand1, operand2, target);
}

fn solve_one(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut pos: usize = 0;
    let input = 1;
    let mut jump = 0;

    let mut operation = 0;
    let mut operand1: usize = 0;
    let mut operand2: usize = 0;
    let mut target: usize = 0;

    while numbers[pos] != 99 {
        if numbers[pos] == 1 {
            operation = 1;
            operand1 = numbers[pos + 1] as usize;
            operand2 = numbers[pos + 2] as usize;
            target = numbers[pos + 3] as usize;
        } else if numbers[pos] == 2 {
            operation = 2;
            operand1 = numbers[pos + 1] as usize;
            operand2 = numbers[pos + 2] as usize;
            target = numbers[pos + 3] as usize;
        } else if numbers[pos] == 3 {
            target = numbers[pos + 1] as usize;
            operation = 3;
        } else if numbers[pos] == 4 {
            operation = 4;
            operand1 = numbers[pos + 1] as usize;
        } else {
            //dbg!(numbers[pos], numbers[pos+1], numbers[pos+2],numbers[pos+3], pos, pos+1,pos+2,pos+3);
            let (op, op1, op2, trt) = parse_parameter_mode(pos, numbers.clone());
            //dbg!(op,op1,op2,trt);
            operation = op as usize;
            operand1 = op1 as usize;
            operand2 = op2 as usize;
            target = trt as usize;
        }

        //dbg!(operation, operand1, operand2, target);
        //println!("---");

        match operation {
            1 => {
                numbers[target] = numbers[operand1] + numbers[operand2];
                jump = 4;
            }
            2 => {
                numbers[target] = numbers[operand1] * numbers[operand2];
                jump = 4;
            }
            3 => {
                numbers[target] = input; //hardcoded input 1
                jump = 2;
            }
            4 => {
                println!("output: {}", numbers[operand1]);
                jump = 2;
            }
            _ => println!("incorrect operation detected at pos: {}", pos),
        }
        pos += jump;
    }
}

fn solve_two(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut pos: usize = 0;
    let input = 5;
    let mut jump = 0;

    let mut operation = 0;
    let mut operand1: i64 = 0;
    let mut operand2: i64 = 0;
    let mut target: i64 = 0;

    while numbers[pos] != 99 {
        dbg!(numbers.clone(), pos);
        if numbers[pos] == 1 {
            operation = 1;
            operand1 = numbers[pos + 1];
            operand2 = numbers[pos + 2];
            target = numbers[pos + 3];
        } else if [2, 7, 8].contains(&numbers[pos]) {
            operation = numbers[pos];
            operand1 = numbers[pos + 1];
            operand2 = numbers[pos + 2];
            target = numbers[pos + 3];
        } else if numbers[pos] == 3 {
            target = numbers[pos + 1];
            operation = 3;
        } else if numbers[pos] == 4 {
            operation = 4;
            operand1 = numbers[pos + 1];
        } else if [5, 6].contains(&numbers[pos]) {
            operation = numbers[pos];
            operand1 = numbers[pos + 1];
            operand2 = numbers[pos + 2];
        } else {
            let (op, op1, op2, trt) = parse_parameter_mode(pos, numbers.clone());
            operation = op;
            operand1 = op1;
            operand2 = op2;
            target = trt;
        }
        /*dbg!(pos, operation, operand1, operand2, target);
        for i in 0..numbers.len() {
            println!("{}\t{}", i, numbers[i]);
        }*/

        match operation {
            1 => {
                numbers[target as usize] = numbers[operand1 as usize] + numbers[operand2 as usize];
                jump = 4;
            }
            2 => {
                numbers[target as usize] = numbers[operand1 as usize] * numbers[operand2 as usize];
                jump = 4;
            }
            3 => {
                numbers[target as usize] = input;
                jump = 2;
            }
            4 => {
                println!("outputted: {}", numbers[operand1 as usize]);
                jump = 2;
            }
            5 => {
                if numbers[operand1 as usize] != 0 {
                    jump = 0;
                    pos = numbers[operand2 as usize] as usize;
                } else {
                    jump = 3;
                }
            }
            6 => {
                if numbers[operand1 as usize] == 0 {
                    jump = 0;
                    pos = numbers[operand2 as usize] as usize;
                } else {
                    jump = 3;
                }
            }
            7 => {
                jump = 4;
                if numbers[operand1 as usize] < numbers[operand2 as usize] {
                    numbers[target as usize] = 1;
                } else {
                    numbers[target as usize] = 0;
                }
            }
            8 => {
                jump = 4;
                if numbers[operand1 as usize] == numbers[operand2 as usize] {
                    numbers[target as usize] = 1;
                } else {
                    numbers[target as usize] = 0;
                }
            }

            _ => println!("incorrect operation detected at pos: {}", pos),
        }
        pos += jump;
    }
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
