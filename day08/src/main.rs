use std::env;
use std::io::{self, BufRead};

fn solve_one(input: Vec<String>) {
    let input = &input[0];
    let mut lines: Vec<&str> = Vec::new();
    let mut i = 0;
    let line_size = 25;
    let layer_size = 6;

    while i < input.len() {
        lines.push(&input[i..i + line_size]);
        i += line_size;
    }
    let mut j = 0;
    let mut min_zeroes = line_size * layer_size;
    let mut answer = 0;
    while j < lines.len() {
        let mut zeroes = 0;
        let mut ones = 0;
        let mut twos = 0;
        for i in 0..layer_size {
            zeroes += lines[j + i].matches("0").count();
            ones += lines[j + i].matches("1").count();
            twos += lines[j + i].matches("2").count();
        }
        if zeroes < min_zeroes {
            min_zeroes = zeroes;
            answer = ones * twos;
        }
        j += layer_size;
    }
    //dbg!(lines.clone());
    dbg!(answer);
}
fn solve_two(input: Vec<String>) {
    let input = &input[0];
    let mut lines: Vec<&str> = Vec::new();
    let mut i = 0;
    let line_size = 25;
    let layer_size = 6;

    while i < input.len() {
        lines.push(&input[i..i + line_size]);
        i += line_size;
    }
    let mut j = 0;

    for i in 0..layer_size {
        for k in 0..line_size {
            j = 0;
            while j < lines.len() {
                if lines[j + i].chars().nth(k).unwrap() != '2' {
                    print!("{}", lines[j + i].chars().nth(k).unwrap());
                    break;
                }
                j += layer_size;
            }
        }
        println!();
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
