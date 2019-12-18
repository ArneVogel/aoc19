extern crate permutohedron;

use permutohedron::Heap;

use std::cmp;
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

fn simulate_loop(
    mut pos: usize,
    mut numbers: Vec<i64>,
    inputs: Vec<i64>,
) -> (usize, Vec<i64>, Vec<i64>, bool) {
    let mut jump = 0;

    let mut operation = 0;
    let mut operand1: i64 = 0;
    let mut operand2: i64 = 0;
    let mut target: i64 = 0;

    let mut input_counter = 0;

    let mut output: Vec<i64> = Vec::new();

    let mut ret = false;

    while numbers[pos] != 99 && !ret {
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
                numbers[target as usize] = inputs[input_counter];
                input_counter += 1;
                jump = 2;
            }
            4 => {
                //println!("outputted: {}", numbers[operand1 as usize]);
                output.push(numbers[operand1 as usize]);
                ret = true;
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
    //dbg!(output.clone());

    return (pos, numbers.clone(), output, numbers[pos] == 99);
}

fn simulate(mut numbers: Vec<i64>, inputs: Vec<i64>) -> Vec<i64> {
    let mut pos: usize = 0;
    let mut jump = 0;

    let mut operation = 0;
    let mut operand1: i64 = 0;
    let mut operand2: i64 = 0;
    let mut target: i64 = 0;

    let mut input_counter = 0;

    let mut output: Vec<i64> = Vec::new();

    while numbers[pos] != 99 {
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
                numbers[target as usize] = inputs[input_counter];
                input_counter += 1;
                jump = 2;
            }
            4 => {
                //println!("outputted: {}", numbers[operand1 as usize]);
                output.push(numbers[operand1 as usize]);
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
    return output;
}

fn solve_one(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let initial_numbers = numbers.clone();

    let mut instructions = [0, 1, 2, 3, 4];

    let mut heap = Heap::new(&mut instructions);
    let mut max = 0;

    while let Some(perm) = heap.next_permutation() {
        numbers = initial_numbers.clone();
        let mut output = 0;
        for p in perm {
            let mut input = Vec::new();
            input.push(p.clone());
            input.push(output.clone());
            let o = simulate(numbers.clone(), input);
            output = o[0];
        }
        max = cmp::max(max, output);
    }
    dbg!(max);
}

fn solve_two(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let initial_numbers = numbers.clone();

    let mut instructions = [5, 6, 7, 8, 9];

    let mut heap = Heap::new(&mut instructions);
    let mut max = 0;
    let mut output = 0;
    while let Some(perm) = heap.next_permutation() {
        let mut machines: Vec<(usize, Vec<i64>, Vec<i64>, bool)> = Vec::new();
        numbers = initial_numbers.clone();

        let mut input = [perm[0], 0];

        let a = simulate_loop(0, numbers.clone(), input.to_vec());
        input[1] = a.2[0];
        input[0] = perm[1];
        //dbg!(a.2[0].clone());
        machines.push(a.clone());

        let a = simulate_loop(0, numbers.clone(), input.to_vec());
        input[1] = a.2[0];
        input[0] = perm[2];
        //dbg!(a.2[0].clone());
        machines.push(a.clone());

        let a = simulate_loop(0, numbers.clone(), input.to_vec());
        input[1] = a.2[0];
        input[0] = perm[3];
        //dbg!(a.2[0].clone());
        machines.push(a.clone());

        let a = simulate_loop(0, numbers.clone(), input.to_vec());
        input[1] = a.2[0];
        input[0] = perm[4];
        //dbg!(a.2[0].clone());
        machines.push(a.clone());

        let a = simulate_loop(0, numbers.clone(), input.to_vec());
        machines.push(a.clone());
        //dbg!(a.2[0].clone());

        let mut i = 4;
        loop {
            let prev = i;
            i = (i + 1) % 5;

            let a = simulate_loop(
                machines[i].0.clone(),
                machines[i].1.clone(),
                machines[prev].2.clone(),
            );

            //dbg!(machines[prev].2.clone());
            machines[i] = a.clone();
            //if (perm[0] == 9 && perm[1] == 8 && perm[2] == 7 && perm[3] == 6 && perm[4] == 5) {
            //dbg!(a.2[0].clone());
            //}
            if a.3 && i == 4 {
                output = machines[4].2[machines[4].2.len() - 1];
                break;
            }
        }
        max = cmp::max(max, output);
    }
    dbg!(max);
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
