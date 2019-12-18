extern crate rayon;
use rayon::prelude::*;
use std::env;
use std::io::{self, BufRead};

fn mask(len: usize, mul: i64) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();
    let base = [0, 1, 0, -1];
    let mut i = 0;
    let mut j = 0;
    while output.len() < len {
        j += 1;
        for _ in 0..mul {
            if j == 1 {
                j += 1;
                continue;
            }
            output.push(base[i % 4]);
        }
        i += 1;
    }
    return output;
}

fn gen_num(num: Vec<i64>) -> Vec<i64> {
    num.par_iter()
        .enumerate()
        .map(|(i, p)| {
            let m = mask(num.len(), (i + 1) as i64);
            let sum: i64 = num.iter().enumerate().map(|(j, d)| d * m[j]).sum();
            (sum % 10).abs()
        })
        .collect()
}

fn gen_num_p2(num: Vec<i64>) -> Vec<i64> {
    let mut output = num.clone();
    for i in (0..num.len()-1).rev() {
        output[i] = (output[i] + output[i+1]) % 10;
    }
    output
}

fn print_offset(num: Vec<i64>, offset: i64) {
    print!("{}", num[(offset + 0) as usize]);
    print!("{}", num[(offset + 1) as usize]);
    print!("{}", num[(offset + 2) as usize]);
    print!("{}", num[(offset + 3) as usize]);
    print!("{}", num[(offset + 4) as usize]);
    print!("{}", num[(offset + 5) as usize]);
    print!("{}", num[(offset + 6) as usize]);
    print!("{}", num[(offset + 7) as usize]);
    println!();
}

fn solve_one(input: Vec<String>) {
    let mut num: Vec<i64> = input[0]
        .chars()
        .map(|x| x.to_string().parse::<i64>().unwrap())
        .collect();
    for _ in 0..100 {
        num = gen_num(num.clone());
    }
    print_offset(num, 0);
}
fn solve_two(input: Vec<String>) {
    let mut num: Vec<i64> = input[0]
        .chars()
        .map(|x| x.to_string().parse::<i64>().unwrap())
        .collect();
    let offset: i64 = input[0][0..7].parse::<i64>().unwrap();
    let mut input: Vec<i64> = Vec::new();
    let mut p = 0;
    for _ in 0..10000 {
        for d in num.iter() {
            if p < offset {
                p += 1;
                continue;
            }

            input.push(*d);
        }
    }
    dbg!(num.len()*10000, offset, num.len()*10000-offset as usize, input.clone().len());
    print_offset(input.clone(),0);

    for i in 0..100 {
        dbg!(i);
        input = gen_num_p2(input.clone());
    }

    print_offset(input, 0);
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
