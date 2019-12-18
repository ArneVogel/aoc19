use std::cmp;
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

fn parse_parameter_mode(pos: usize, base: i64, numbers: Vec<i64>) -> (i64, i64, i64, i64) {
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
    } else if digits.len() >= 3 && digits.get(digits.len() - 3).unwrap() == &2 {
        operand1 = numbers[pos + 1] + base;
    } else {
        operand1 = numbers[pos + 1];
    }

    if digits.len() >= 4 && digits.get(digits.len() - 4).unwrap() == &1 {
        operand2 = (pos + 2) as i64;
    } else if digits.len() >= 4 && digits.get(digits.len() - 4).unwrap() == &2 {
        operand2 = numbers[pos + 2] + base;
    } else {
        operand2 = numbers[pos + 2];
    }

    if digits.len() >= 5 && digits.get(digits.len() - 5).unwrap() == &1 {
        target = (pos + 3) as i64;
    } else if digits.len() >= 5 && digits.get(digits.len() - 5).unwrap() == &2 {
        target = numbers[pos + 3] + base;
    } else {
        target = numbers[pos + 3];
    }

    return (operation as i64, operand1, operand2, target);
}

fn simulate_day13_p2(
    mut numbers: Vec<i64>,
    inputs: Vec<i64>,
    mut pos: usize,
    mut base: i64,
    max_outputs: i64,
) -> (Vec<i64>, Vec<i64>, usize, i64, bool) {
    let mut jump = 0;
    let mut num_outputs = 0;

    let mut operation = 0;
    let mut operand1: i64 = 0;
    let mut operand2: i64 = 0;
    let mut target: i64 = 0;

    let mut input_counter = 0;

    let mut output: Vec<i64> = Vec::new();

    let numbers_size = numbers.len();

    // fill with zeroes to allow access out of bounds of original size
    if !(numbers[numbers.len() - 1] == 0
        && numbers[numbers.len() - 2] == 0
        && numbers[numbers.len() - 3] == 0)
    {
        for i in 0..numbers_size * 6 {
            numbers.push(0);
        }
    }

    while numbers[pos] != 99 && (max_outputs == -1 || num_outputs != max_outputs) {
        if output.len() >= 3 {
            let l = output.len();
            if output[l - 3] == -1 && output[l - 2] == 0 {
                break;
            }
        }
        //dbg!(numbers.clone(), pos, numbers[pos]);
        if [1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&numbers[pos]) {
            operation = numbers[pos];
            operand1 = numbers[pos + 1];
            operand2 = numbers[pos + 2];
            target = numbers[pos + 3];
        } else {
            let (op, op1, op2, trt) = parse_parameter_mode(pos, base, numbers.clone());
            operation = op;
            operand1 = op1;
            operand2 = op2;
            target = trt;
        }
        //dbg!(operation, operand1, operand2, target);

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
                numbers[operand1 as usize] = inputs[input_counter];
                input_counter += 1;
                jump = 2;
            }
            4 => {
                //println!("outputted: {}", numbers[operand1 as usize]);
                output.push(numbers[operand1 as usize]);
                num_outputs += 1;
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
            9 => {
                jump = 2;
                base += numbers[operand1 as usize];
            }

            _ => println!("incorrect operation detected at pos: {}", pos),
        }
        pos += jump;
    }
    let finished = numbers[pos] == 99;
    return (output, numbers, pos, base, finished);
}

fn simulate(
    mut numbers: Vec<i64>,
    inputs: Vec<i64>,
    mut pos: usize,
    mut base: i64,
    max_outputs: i64,
) -> (Vec<i64>, Vec<i64>, usize, i64, bool) {
    let mut jump = 0;
    let mut num_outputs = 0;

    let mut operation = 0;
    let mut operand1: i64 = 0;
    let mut operand2: i64 = 0;
    let mut target: i64 = 0;

    let mut input_counter = 0;

    let mut output: Vec<i64> = Vec::new();

    let numbers_size = numbers.len();

    // fill with zeroes to allow access out of bounds of original size
    if !(numbers[numbers.len() - 1] == 0
        && numbers[numbers.len() - 2] == 0
        && numbers[numbers.len() - 3] == 0)
    {
        for i in 0..numbers_size * 6 {
            numbers.push(0);
        }
    }

    while numbers[pos] != 99 && (max_outputs == -1 || num_outputs != max_outputs) {
        //dbg!(numbers.clone(), pos, numbers[pos]);
        if [1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&numbers[pos]) {
            operation = numbers[pos];
            operand1 = numbers[pos + 1];
            operand2 = numbers[pos + 2];
            target = numbers[pos + 3];
        } else {
            let (op, op1, op2, trt) = parse_parameter_mode(pos, base, numbers.clone());
            operation = op;
            operand1 = op1;
            operand2 = op2;
            target = trt;
        }
        //dbg!(operation, operand1, operand2, target);

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
                numbers[operand1 as usize] = inputs[input_counter];
                input_counter += 1;
                jump = 2;
            }
            4 => {
                //println!("outputted: {}", numbers[operand1 as usize]);
                output.push(numbers[operand1 as usize]);
                num_outputs += 1;
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
            9 => {
                jump = 2;
                base += numbers[operand1 as usize];
            }

            _ => println!("incorrect operation detected at pos: {}", pos),
        }
        pos += jump;
    }
    let finished = numbers[pos] == 99;
    return (output, numbers, pos, base, finished);
}

fn solve_one(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let (output, _, _, _, _) = simulate(numbers, [].to_vec(), 0, 0, -1);

    let mut tiles: HashMap<[i64; 2], i64> = HashMap::new();
    for i in (0..output.len()).step_by(3) {
        let t = tiles.entry([output[i], output[i + 1]]).or_insert(0);
        *t = output[i + 2];
    }

    let mut sum = 0;
    for (_, t) in &tiles {
        if *t == 2 {
            sum += 1;
        }
    }
    dbg!(sum);
}

fn print_tiles(tiles: HashMap<[i64; 2], i64>) {
    let mut max_x = 0;
    let mut max_y = 0;
    for (t, _) in tiles.clone() {
        max_x = cmp::max(max_x, t[0]);
        max_y = cmp::max(max_y, t[1]);
    }
    max_x += 1;
    max_y += 1;

    let mut p: Vec<Vec<i64>> = Vec::new();
    for _ in 0..max_y {
        let mut line: Vec<i64> = Vec::new();
        for _ in 0..max_x {
            line.push(0);
        }
        p.push(line);
    }
    for (t, m) in tiles {
        p[t[1] as usize][t[0] as usize] = m;
    }

    for y in 0..max_y {
        for x in 0..max_x {
            match p[y as usize][x as usize] {
                0 => print!(" "),
                1 => print!("w"),
                2 => print!("b"),
                3 => print!("-"),
                4 => print!("o"),
                _ => panic!("wrong tile"),
            }
        }
        println!();
    }
}

fn position(tiles: HashMap<[i64; 2], i64>, target: i64) -> i64 {
    for (t, p) in tiles {
        if p == target {
            return t[0];
        }
    }
    return -1;
}

fn solve_two(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    numbers[0] = 2;

    let mut tiles: HashMap<[i64; 2], i64> = HashMap::new();
    let mut score = 0;
    let mut pos = 0;
    let mut base = 0;
    let mut ball_pos = 0;
    let mut paddle_pos = 0;
    let mut direction = 0;
    loop {
        let (output, n, p, b, done) = simulate_day13_p2(numbers, [direction].to_vec(), pos, base, 3);
        numbers = n;
        pos = p;
        base = b;

        for i in (0..output.len()).step_by(3) {
            if output[i] == -1 && output[i + 1] == 0 {
                score = output[i + 2];
                continue;
            }
            let t = tiles.entry([output[i], output[i + 1]]).or_insert(0);
            *t = output[i + 2];
        }

        if position(tiles.clone(), 3) != -1 {
            paddle_pos = position(tiles.clone(), 3);
        }
        if position(tiles.clone(), 4) != -1 {
            ball_pos = position(tiles.clone(), 4);
        }

        if paddle_pos == ball_pos {
            direction = 0;
        } else if paddle_pos < ball_pos {
            direction = 1;
        } else {
            direction = -1;
        }

        print_tiles(tiles.clone());
        if done {
            break;
        }
    }
    dbg!(score);
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
