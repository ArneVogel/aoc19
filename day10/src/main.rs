extern crate num;
use crate::num::integer::Roots;
use crate::num::Integer;
use std::cmp;
use std::env;
use std::io::{self, BufRead};

fn can_see(a: [i64; 2], b: [i64; 2], astroids: Vec<[i64; 2]>) -> bool {
    if a[0] == b[0] && a[1] == b[1] {
        return false;
    }
    let max_y = cmp::max(a[1], b[1]);
    let min_y = cmp::min(a[1], b[1]);

    let max_x = cmp::max(a[0], b[0]);
    let min_x = cmp::min(a[0], b[0]);

    let mut diff_y = max_y - min_y;
    let mut diff_x = max_x - min_x;

    let gcd = diff_y.gcd(&diff_x);
    diff_y /= gcd;
    diff_x /= gcd;
    //dbg!(a, b);
    //dbg!(min_y, max_y, min_x, max_x, diff_y, diff_x);
    if diff_y == 0 {
        for j in (min_x + diff_x..max_x).step_by(diff_x as usize) {
            if astroids.contains(&[j, a[1]]) {
                //dbg!("same line");
                return false;
            }
        }
    }
    if diff_x == 0 {
        for i in (min_y + diff_y..max_y).step_by(diff_y as usize) {
            if astroids.contains(&[a[0], i]) {
                //dbg!("same line");
                return false;
            }
        }
    }
    if diff_x == 0 || diff_y == 0 {
        return true;
    }
    let mut i = 1;

    for (i, y) in (min_y + diff_y..max_y).step_by(diff_y as usize).enumerate() {
        let x = min_x + diff_x * (i + 1) as i64;
        //dbg!(x, y);
        if astroids.contains(&[x, y]) {
            //dbg!("same diag");
            return false;
        }
    }
    return true;
}

// close enough, solution +- 3
fn solve_one(input: Vec<String>) {
    let mut astroids: Vec<[i64; 2]> = Vec::new();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                astroids.push([i as i64, j as i64]);
            }
        }
    }
    let mut max = 0;
    let mut max_astr = [0, 0];
    for astroid in &astroids {
        let mut seeing = 0;
        for a in &astroids {
            if can_see(*astroid, *a, astroids.clone()) {
                seeing += 1;
            }
        }
        max = cmp::max(seeing, max);
        if max == seeing {
            max_astr = *astroid;
        }
        //dbg!(astroid, seeing);
    }
    dbg!(max);
    dbg!(max_astr);
}

fn angle(c: [i64; 2], d: [i64; 2], middle: [i64; 2]) -> f64 {
    let TWOPI = 6.2831853071795865;
    let RAD2DEG = 57.2957795130823209;
    let mut a = c;
    let mut b = d;
    a[0] -= middle[0];
    b[0] -= middle[0];
    a[1] -= middle[1];
    b[1] -= middle[1];

    let mut diff_y = (a[0] - b[0]) as f64;
    let mut diff_x = (a[0] - b[1]) as f64;

    let mut theta = diff_y.atan2(diff_x);
    if theta < 0.0 {
        theta += TWOPI;
    }
    return theta * RAD2DEG;
}

fn distance(a: [i64; 2], b: [i64; 2]) -> f64 {
    let p = (a[0] - b[0]).pow(2) as f64;
    let q = (a[1] - b[1]).pow(2) as f64;
    return ((p + q) as f64).sqrt();
}

fn solve_two(input: Vec<String>) {
    let mut astroids: Vec<[i64; 2]> = Vec::new();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                astroids.push([i as i64, j as i64]);
            }
        }
    }
    //dbg!(astroids.clone());

    // middle for input2 is 20,20
    let middle = [14, 19];
    let mut curr_angle = 90.0;
    let mut counter = 1;
    while astroids.len() != 0 {
        counter += 1;
        let mut curr_min_angle = 20000.0;
        let mut curr_delete = [1000, 1000];
        let mut curr_index = 0;
        let mut max_angle = 0.0;
        for (i, a) in astroids.iter().enumerate() {
            //dbg!(*a, angle(middle, *a, middle));
            max_angle = f64::max(max_angle, angle(middle, *a, middle));

            if angle(middle, *a, middle) >= curr_angle
                && angle(middle, *a, middle) <= curr_min_angle
            {
                if angle(middle, *a, middle) < curr_min_angle
                    || distance(middle, *a) < distance(middle, curr_delete)
                {
                    curr_min_angle = angle(middle, *a, middle);
                    curr_delete = *a;
                    curr_index = i;
                }
            }
        }
        curr_angle = angle(middle, curr_delete, middle) + 0.000001;
        if max_angle <= curr_angle {
            curr_angle = 0.0;
        }
        if counter == 201 {
            dbg!(curr_delete[1] * 100 + curr_delete[0]);
        }
        //dbg!(curr_delete, curr_angle, counter, astroids.len());
        astroids.remove(curr_index);
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
