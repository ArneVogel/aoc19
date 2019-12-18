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

    while numbers[pos] != 99 && num_outputs != max_outputs {
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
                println!("outputted: {}", numbers[operand1 as usize]);
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

fn draw_field(field: Vec<Vec<[i64; 2]>>, pos: [usize; 2], direction: usize) -> i64 {
    let mut sum = 0;
    for (i, line) in field.iter().enumerate() {
        for (j, pixel) in line.iter().enumerate() {
            if pixel[1] != 0 {
                sum += 1;
            }
            if i == pos[0] && j == pos[1] {
                match direction {
                    1 => print!("^"),
                    2 => print!(">"),
                    3 => print!("v"),
                    4 => print!("<"),
                    _ => panic!("wrong direction"),
                }
                continue;
            }
            if pixel[0] == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    return sum;
}

fn solve_one(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut pos = [50, 50];
    //   1
    //  4 2
    //   3
    let mut direction = 1;
    let field_dimensions = 130;
    let mut field: Vec<Vec<[i64; 2]>> = Vec::new();
    for _ in 0..field_dimensions {
        let mut line: Vec<[i64; 2]> = Vec::new();
        for _ in 0..field_dimensions {
            line.push([0, 0]); //color 0 black 1 white, visited
        }
        field.push(line);
    }

    let mut standing_on_color = 0;
    let mut int_pos = 0;
    let mut base : i64 = 0;
    loop {
        let (o, n, p, bs, b) = simulate(numbers.clone(), [standing_on_color].to_vec(), int_pos, base, 2);
        if b {
            break;
        }
        int_pos = p;
        base = bs;
        numbers = n;

        let mut v = field[pos[0]][pos[1]];
        //set new color
        v[0] = o[0];
        //increase visit count
        v[1] += 1;
        field[pos[0]][pos[1]] = v;

        if o[1] == 0 {
            direction -= 1;
            if direction == 0 {
                direction = 4
            }
        } else {
            direction += 1;
            if direction == 5 {
                direction = 1
            }
        }

        match direction {
            1 => pos[0] -= 1,
            2 => pos[1] += 1,
            3 => pos[0] += 1,
            4 => pos[1] -= 1,
            _ => panic!("wrong direction"),
        }
        standing_on_color = field[pos[0]][pos[1]][0];
    }
    dbg!(draw_field(field.clone(), pos.clone(), direction));
}

fn solve_two(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut pos = [0, 0];
    //   1
    //  4 2
    //   3
    let mut direction = 1;
    let field_dimensions = 60;
    let mut field: Vec<Vec<[i64; 2]>> = Vec::new();
    for _ in 0..field_dimensions {
        let mut line: Vec<[i64; 2]> = Vec::new();
        for _ in 0..field_dimensions {
            line.push([0, 0]); //color 0 black 1 white, visited
        }
        field.push(line);
    }

    let mut standing_on_color = 1;
    let mut int_pos = 0;
    let mut base : i64 = 0;
    loop {
        let (o, n, p, bs, b) = simulate(numbers.clone(), [standing_on_color].to_vec(), int_pos, base, 2);
        if b {
            break;
        }
        int_pos = p;
        base = bs;
        numbers = n;

        let mut v = field[pos[0]][pos[1]];
        //set new color
        v[0] = o[0];
        //increase visit count
        v[1] += 1;
        field[pos[0]][pos[1]] = v;

        if o[1] == 0 {
            direction -= 1;
            if direction == 0 {
                direction = 4
            }
        } else {
            direction += 1;
            if direction == 5 {
                direction = 1
            }
        }

        match direction {
            1 => pos[0] -= 1,
            2 => pos[1] += 1,
            3 => pos[0] += 1,
            4 => pos[1] -= 1,
            _ => panic!("wrong direction"),
        }
        standing_on_color = field[pos[0]][pos[1]][0];
    }
    dbg!(draw_field(field.clone(), pos.clone(), direction));
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
