use std::env;
use std::io::{self, BufRead};

fn solve_one(input: Vec<String>) {
    let orbits: Vec<Vec<&str>> = input.iter().map(|x| x.split(")").collect()).collect();
    let orbits_len = orbits.len();
    let mut sum = 0;
    let mut l = true;
    for orbit in orbits.clone() {
        //dbg!(orbit);
        let mut outer = orbit[1];
        let mut inner = orbit[0];
        let mut i = 0;
        if orbit[0] == "COM" {
            sum += 1;
            l = false;
        }
        while l {
            if orbits[i][0] == "COM" && orbits[i][1] == inner {
                sum += 2;
                break;
            }
            if orbits[i][1] == inner {
                inner = orbits[i][0];
                sum += 1;
            }
            i += 1;
            i %= orbits_len;
        }
        l = true;
    }
    dbg!(sum);
}

fn distance_from_com(orbits: Vec<Vec<&str>>, inner: &str) -> (i64, Vec<String>) {
    let mut sum = 0;
    let orbits_len = orbits.len();
    let mut i = 0;
    let mut my_inner = inner;
    let mut planets : Vec<String> = Vec::new();
    loop {
        if orbits[i][0] == "COM" && orbits[i][1] == my_inner {
            sum += 2;
            planets.push(orbits[i][1].to_string());
            break;
        }
        if orbits[i][1] == my_inner {
            planets.push(my_inner.to_string());
            my_inner = orbits[i][0];
            sum += 1;
        }
        i += 1;
        i %= orbits_len;
    }
    return (sum, planets);
}

fn solve_two(input: Vec<String>) {
    let orbits: Vec<Vec<&str>> = input.iter().map(|x| x.split(")").collect()).collect();
    let orbits_len = orbits.len();
    let mut sum = 0;
    let mut planets : Vec<Vec<String>> = Vec::new();
    for orbit in orbits.clone() {
        if orbit[1] == "SAN" || orbit[1] == "YOU" {
            let inner = orbit[0];
            let (s,p) = distance_from_com(orbits.clone(), inner);
            sum += s;
            planets.push(p);
        }
    }
    planets[0].reverse();
    planets[1].reverse();
    let mut differ = 0;
    let mut differ_planet = "".to_string();
    for (i,p) in planets[0].iter().enumerate() {
        if (p != &planets[1][i]) {
            differ = i;
            differ_planet = planets[1][i-1].clone();
            break;
        }
    }
    sum -= (differ * 2) as i64;
    sum -= 2 as i64;
    dbg!(differ);
    dbg!(differ_planet);
    dbg!(planets);
    dbg!(sum);
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
