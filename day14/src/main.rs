use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};

fn solve_one(input: Vec<String>) {
    let mut requirements: HashMap<String, (i64, HashSet<(i64, String)>)> = HashMap::new();
    for l in input {
        let equal_split = l.split(" => ").collect::<Vec<&str>>();

        let key_parts = equal_split[1].split(" ").collect::<Vec<&str>>();
        let mut key = key_parts[1].to_string();
        let key_quant = key_parts[0].parse::<i64>().unwrap();

        requirements.insert(key.clone(), (key_quant, HashSet::new()));
        for v in equal_split[0].split(", ").collect::<Vec<&str>>().iter() {
            let value_parts = v.split(" ").collect::<Vec<&str>>();
            let value = (
                value_parts[0].parse::<i64>().unwrap(),
                value_parts[1].to_string(),
            );
            requirements.get_mut(&key).unwrap().1.insert(value);
        }
    }

    dbg!(ore_required(requirements.clone(), 1));
    dbg!(ore_required(requirements.clone(), 628597));
    dbg!(ore_required(requirements.clone(), 1184208));
    dbg!(ore_required(requirements.clone(), 1184209));
    dbg!(ore_required(requirements.clone(), 1184210));
    dbg!(ore_required(requirements.clone(), 844446));
    dbg!(ore_required(requirements.clone(), 844447));
}

fn ore_required(requirements: HashMap<String, (i64, HashSet<(i64, String)>)>, target: i64) -> i64 {
    let mut required: HashMap<String, i64> = HashMap::new();
    let mut overpaid: HashMap<String, i64> = HashMap::new();
    for r in requirements.get("FUEL").unwrap().1.iter() {
        required.insert(r.1.clone(), r.0 * target);
    }
    let mut sum = 0;
    while !required.is_empty() {
        let required_fuel: String = required.keys().collect::<Vec<&String>>()[0].to_string();
        let required_amount = required.remove(&required_fuel).unwrap();
        let min_buy = requirements.get(&required_fuel).unwrap().0;
        let mut multiple = 0;

        let target_buy =
            required_amount - overpaid.entry(required_fuel.clone()).or_insert(0).clone();
        multiple = 0;
        while multiple * min_buy < target_buy {
            multiple += 1;
        }
        let to_buy = multiple * min_buy;

        for re in requirements.get(&required_fuel).unwrap().1.iter() {
            if re.1 != "ORE" {
                let requirement = required.entry(re.1.clone()).or_insert(0);
                let buying = multiple * re.0;
                *requirement += buying;
            } else {
                let buying = multiple * re.0;
                sum += buying;
            }
        }
        let over = to_buy - target_buy;
        overpaid.insert(required_fuel.clone(), over);
    }
    sum
}

fn solve_two(input: Vec<String>) {
    let mut requirements: HashMap<String, (i64, HashSet<(i64, String)>)> = HashMap::new();
    for l in input {
        let equal_split = l.split(" => ").collect::<Vec<&str>>();

        let key_parts = equal_split[1].split(" ").collect::<Vec<&str>>();
        let mut key = key_parts[1].to_string();
        let key_quant = key_parts[0].parse::<i64>().unwrap();

        requirements.insert(key.clone(), (key_quant, HashSet::new()));
        for v in equal_split[0].split(", ").collect::<Vec<&str>>().iter() {
            let value_parts = v.split(" ").collect::<Vec<&str>>();
            let value = (
                value_parts[0].parse::<i64>().unwrap(),
                value_parts[1].to_string(),
            );
            requirements.get_mut(&key).unwrap().1.insert(value);
        }
    }

    let target = 1000000000000;
    let mut ore = 0;
    let mut fuel = 1;
    let mut ore_per_fuel = 0;

    //works for both inputs but relies on fast convergence

    ore = ore_required(requirements.clone(), fuel);
    dbg!(ore,fuel);
    ore_per_fuel = ore/fuel;
    let mut fuel = target/ore_per_fuel;

    ore = ore_required(requirements.clone(), fuel);
    ore_per_fuel = ore/fuel;
    dbg!(ore,fuel);
    let mut fuel = target/ore_per_fuel;
    
    ore = ore_required(requirements.clone(), fuel);
    ore_per_fuel = ore/fuel;
    dbg!(ore,fuel);
    let mut fuel = target/ore_per_fuel;

    ore = ore_required(requirements.clone(), fuel);
    ore_per_fuel = ore/fuel;
    dbg!(ore,fuel);
    let mut fuel = target/ore_per_fuel;
    
    while ore > target {
        fuel -= 1;
        ore = ore_required(requirements.clone(), fuel);
    }
    dbg!(fuel);
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
