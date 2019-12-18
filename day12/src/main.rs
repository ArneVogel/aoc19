extern crate num;

use crate::num::Integer;
use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

fn generate_gravity(positions: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut gravity: Vec<Vec<i64>> = Vec::new();
    for (i, planet) in positions.iter().enumerate() {
        let mut g: Vec<i64> = [0, 0, 0].to_vec();
        for (j, p) in positions.iter().enumerate() {
            if i == j {
                continue;
            }
            for k in 0..3 {
                if planet[k] > p[k] {
                    g[k] -= 1;
                } else if planet[k] < p[k] {
                    g[k] += 1;
                }
            }
        }
        gravity.push(g);
    }
    return gravity;
}

fn add_vec(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let mut c: Vec<i64> = [0, 0, 0].to_vec();
    for (i, (aval, bval)) in a.iter().zip(&b).enumerate() {
        c[i] = aval + bval;
    }
    return c;
}

fn add_matrix(a: Vec<Vec<i64>>, b: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut c: Vec<Vec<i64>> = Vec::new();
    for (i, (avec, bvec)) in a.iter().zip(&b).enumerate() {
        c.push(add_vec(avec.to_vec(), bvec.to_vec()));
    }
    return c;
}

fn energy(a: Vec<Vec<i64>>, b: Vec<Vec<i64>>) -> i64 {
    let a_pot: Vec<i64> = a
        .iter()
        .map(|x| x[0].abs() + x[1].abs() + x[2].abs())
        .collect();
    let b_kin: Vec<i64> = b
        .iter()
        .map(|x| x[0].abs() + x[1].abs() + x[2].abs())
        .collect();
    let mut sum = 0;
    for (_, (aval, bval)) in a_pot.iter().zip(&b_kin).enumerate() {
        sum += aval * bval;
    }
    return sum;
}

fn solve_one(input: Vec<String>) {
    let mut positions: Vec<Vec<i64>> = Vec::new();
    let mut velocities: Vec<Vec<i64>> = Vec::new();
    let mut gravity: Vec<Vec<i64>> = Vec::new();
    for i in input {
        velocities.push([0, 0, 0].to_vec());
        gravity.push([0, 0, 0].to_vec());
        positions.push(i.split(",").map(|x| x.parse::<i64>().unwrap()).collect());
    }

    for _ in 0..1000 {
        gravity = generate_gravity(positions.clone());
        positions = add_matrix(positions.clone(), velocities.clone());
        positions = add_matrix(positions.clone(), gravity.clone());
        velocities = add_matrix(gravity.clone(), velocities.clone());
    }
    dbg!(energy(positions, velocities));
}

fn axis(a: Vec<Vec<i64>>, b: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut axis: Vec<Vec<i64>> = Vec::new();
    for _ in 0..3 {
        axis.push([].to_vec());
    }
    for avec in a.iter() {
        for (i, e) in avec.iter().enumerate() {
            axis[i].push(*e);
        }
    }

    for bvec in b.iter() {
        for (i, e) in bvec.iter().enumerate() {
            axis[i].push(*e);
        }
    }

    return axis;
}

fn solve_two(input: Vec<String>) {
    let mut positions: Vec<Vec<i64>> = Vec::new();
    let mut velocities: Vec<Vec<i64>> = Vec::new();
    let mut gravity: Vec<Vec<i64>> = Vec::new();

    for i in input {
        velocities.push([0, 0, 0].to_vec());
        gravity.push([0, 0, 0].to_vec());
        positions.push(i.split(",").map(|x| x.parse::<i64>().unwrap()).collect());
    }

    // if a repeating position for all axis has been found the result is the lcm of the length of
    // the repeating cycles
    let mut sets: Vec<HashSet<Vec<i64>>> = Vec::new();
    for _ in 0..3 {
        sets.push(HashSet::new());
    }

    loop {
        let a = axis(positions.clone(), velocities.clone());
        if sets[0].contains(&a[0]) && sets[1].contains(&a[1]) && sets[2].contains(&a[2]) {
            break;
        } else {
            sets[0].insert(a[0].clone());
            sets[1].insert(a[1].clone());
            sets[2].insert(a[2].clone());
        }
        gravity = generate_gravity(positions.clone());
        positions = add_matrix(positions.clone(), velocities.clone());
        positions = add_matrix(positions.clone(), gravity.clone());
        velocities = add_matrix(gravity.clone(), velocities.clone());
    }
    let x = sets[0].len() as i64;
    let y = sets[1].len() as i64;
    let z = sets[2].len() as i64;
    dbg!(x.lcm(&y.lcm(&z)));
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
