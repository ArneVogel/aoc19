use math::round;
use std::env;
use std::io::{self, BufRead};

fn solve_one(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut pos: usize = 0;
    dbg!(numbers.clone());
    while numbers[pos] != 99 {
        let copy = numbers.clone();
        if numbers[pos] == 1 {
            numbers[copy[pos + 3] as usize] =
                numbers[copy[pos + 1] as usize] + numbers[copy[pos + 2] as usize]
        } else if numbers[pos] == 2 {
            numbers[copy[pos + 3] as usize] =
                numbers[copy[pos + 1] as usize] * numbers[copy[pos + 2] as usize]
        }
        pos += 4;
    }
    println!("{:?}", numbers);
}

fn solve_two(input: Vec<String>) {
    let mut numbers: Vec<i64> = input[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let backup = numbers.clone();
    for i in 0..99 {
        for j in 0..99 {
            numbers = backup.clone();
            numbers[1] = i;
            numbers[2] = j;
            let mut pos: usize = 0;
            while numbers[pos] != 99 {
                let copy = numbers.clone();
                if numbers[pos] == 1 {
                    numbers[copy[pos + 3] as usize] =
                        numbers[copy[pos + 1] as usize] + numbers[copy[pos + 2] as usize]
                } else if numbers[pos] == 2 {
                    numbers[copy[pos + 3] as usize] =
                        numbers[copy[pos + 1] as usize] * numbers[copy[pos + 2] as usize]
                }
                pos += 4;
            }

            if numbers[0] == 19690720 {
                dbg!(i);
                dbg!(j);
            }
        }
    }
    println!("{:?}", numbers);
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
