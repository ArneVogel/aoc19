use math::round;
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

fn distance(x: i64, y: i64) -> i64 {
    x.abs() + y.abs()
}

fn solve_one(input: Vec<String>) {
    let mut points = HashMap::new();
    for j in 0..2 {
        let numbers: Vec<&str> = input[j].split(",").collect();
        let mut x = 0;
        let mut y = 0;
        for num in numbers.clone() {
            let dir = num.get(0..1).unwrap();
            let length = num.get(1..num.len()).unwrap().parse::<i64>().unwrap();
            match dir {
                "R" => {
                    for i in (x + 1)..(x + length + 1) {
                        let t = points.entry([i, y]).or_insert([0, 0]);
                        t[j] += 1;
                    }
                    x += length
                }
                "L" => {
                    for i in (x - length)..(x) {
                        let t = points.entry([i, y]).or_insert([0, 0]);
                        t[j] += 1;
                    }
                    x -= length
                }
                "U" => {
                    for i in (y + 1)..(y + length + 1) {
                        let t = points.entry([x, i]).or_insert([0, 0]);
                        t[j] += 1;
                    }
                    y += length
                }
                "D" => {
                    for i in (y - length)..(y + 1) {
                        let t = points.entry([x, i]).or_insert([0, 0]);
                        t[j] += 1;
                    }
                    y -= length
                }
                _ => println!("what???"),
            }
        }
    }

    let mut min = 1000000000;
    for (key, value) in &points {
        if value[0] >= 1 && value[1] >= 1 {
            let distance = distance(key[0], key[1]);
            if (distance > 0 && distance < min) {
                min = distance;
            }
        }
    }
    dbg!(min);
}

fn solve_two(input: Vec<String>) {
    let mut points = HashMap::new();
    for j in 0..2 {
        let numbers: Vec<&str> = input[j].split(",").collect();
        let mut x = 0;
        let mut y = 0;
        let mut steps = 0;
        for num in numbers.clone() {
            //dbg!(num);
            let tmp = steps;
            let dir = num.get(0..1).unwrap();
            let length = num.get(1..num.len()).unwrap().parse::<i64>().unwrap();
            match dir {
                "R" => {
                    for i in (x + 1)..(x + length + 1) {
                        steps += 1;
                        let t = points.entry([i, y]).or_insert([0, 0, 0, 0]);
                        //dbg!(i,y);
                        t[j] += 1;
                        if t[j + 2] == 0 {
                            t[j + 2] = steps;
                        }
                    }
                    x += length
                }
                "L" => {
                    for i in ((x - length)..(x)).rev() {
                        steps += 1;
                        let t = points.entry([i, y]).or_insert([0, 0, 0, 0]);
                        //dbg!(i,y);
                        t[j] += 1;
                        if t[j + 2] == 0 {
                            t[j + 2] = steps;
                        }
                    }
                    x -= length
                }
                "U" => {
                    for i in (y + 1)..(y + length + 1) {
                        steps += 1;
                        let t = points.entry([x, i]).or_insert([0, 0, 0, 0]);
                        //dbg!(x,i);
                        t[j] += 1;
                        if t[j + 2] == 0 {
                            t[j + 2] = steps;
                        }
                    }
                    y += length
                }
                "D" => {
                    for i in ((y - length)..(y)) {
                        steps += 1;
                        let t = points.entry([x, i]).or_insert([0, 0, 0, 0]);
                        //dbg!(x,i);
                        t[j] += 1;
                        if t[j + 2] == 0 {
                            t[j + 2] = steps;
                        }
                    }
                    y -= length
                }
                _ => println!("what???"),
            }
            dbg!(steps);
            if (tmp+length != steps) {
                dbg!("error", num);
            }
        }
    }

    let mut min = 1000000000;
    for (key, value) in &points {
        if value[0] >= 1 && value[1] >= 1 {
            dbg!(key);
            dbg!(value);
            let distance = value[2] + value[3];
            if (distance > 0 && distance < min) {
                min = distance;
            }
        }
    }
    dbg!(min);
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
