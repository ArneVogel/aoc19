use std::env;
use std::io::{self, BufRead};

fn solved(maze: Vec<String>) -> bool {
    for line in maze {
        for c in line.chars() {
            if c != '#' && c != '@' && c != '.' {
                return false;
            }
        }
    }
    true
}

fn robot_pos(maze: Vec<String>) -> (usize, usize) {
    for (i, line) in maze.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

fn moves(maze: Vec<String>) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();
    let pos = robot_pos(maze.clone());
    let keys = ['a', 'b', 'c'];

    if pos.0 >= 1 {
        if maze[(pos.0) - 1].chars().nth(pos.1).unwrap() == '.' {
            let mut clone = maze.clone();
            clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
            clone[pos.0 - 1].replace_range(pos.1..pos.1 + 1, "@");
            output.push(clone);
        } else if keys.contains(&maze[(pos.0) - 1].chars().nth(pos.1).unwrap()) {
            let mut clone = maze.clone();
            clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
            let c = clone[pos.0 - 1]
                .chars()
                .nth(pos.1)
                .unwrap()
                .to_uppercase()
                .to_string()
                .chars()
                .nth(0)
                .unwrap();
            for (i, line) in clone.clone().iter().enumerate() {
                clone[i] = line.replace(c, ".");
            }
            output.push(clone);
        }
    }
    if pos.0 < maze.len() {
        if maze[(pos.0) + 1].chars().nth(pos.1).unwrap() == '.' {
            let mut clone = maze.clone();
            clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
            clone[pos.0 + 1].replace_range(pos.1..pos.1 + 1, "@");
            output.push(clone);
        } else if keys.contains(&maze[(pos.0) + 1].chars().nth(pos.1).unwrap()) {
            let mut clone = maze.clone();
            clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
            let c = clone[pos.0 + 1]
                .chars()
                .nth(pos.1)
                .unwrap()
                .to_uppercase()
                .to_string()
                .chars()
                .nth(0)
                .unwrap();
            for (i, line) in clone.clone().iter().enumerate() {
                clone[i] = line.replace(c, ".");
            }
            output.push(clone);
        }
    }
    if pos.1 >= 1 {
        if maze[(pos.0) ].chars().nth(pos.1-1).unwrap() == '.' {
            let mut clone = maze.clone();
            clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
            clone[pos.0 - 1].replace_range(pos.1..pos.1 + 1, "@");
            output.push(clone);
        } else if keys.contains(&maze[(pos.0) - 1].chars().nth(pos.1).unwrap()) {
            let mut clone = maze.clone();
            clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
            let c = clone[pos.0 - 1]
                .chars()
                .nth(pos.1)
                .unwrap()
                .to_uppercase()
                .to_string()
                .chars()
                .nth(0)
                .unwrap();
            for (i, line) in clone.clone().iter().enumerate() {
                clone[i] = line.replace(c, ".");
            }
            output.push(clone);
        }
    }
    if pos.0 < maze.len() {
        if maze[(pos.0) + 1].chars().nth(pos.1).unwrap() == '.' {
            let mut clone = maze.clone();
            clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
            clone[pos.0 + 1].replace_range(pos.1..pos.1 + 1, "@");
            output.push(clone);
        } else if keys.contains(&maze[(pos.0) + 1].chars().nth(pos.1).unwrap()) {
            let mut clone = maze.clone();
            clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
            let c = clone[pos.0 + 1]
                .chars()
                .nth(pos.1)
                .unwrap()
                .to_uppercase()
                .to_string()
                .chars()
                .nth(0)
                .unwrap();
            for (i, line) in clone.clone().iter().enumerate() {
                clone[i] = line.replace(c, ".");
            }
            output.push(clone);
        }
    }

    return output;
}

fn print_maze(maze: Vec<String>) {
    for line in maze {
        println!("{}", line);
    }
}

fn solve_one(input: Vec<String>) {
    let mut robots: Vec<Vec<String>> = Vec::new();
    robots.push(input);
    let mut sum = 0;

    loop {
        sum += 1;
        let mut old_robots = robots.clone();
        robots.clear();
        for robot in old_robots {
            if solved(robot.clone()) {
                dbg!(sum);
                return;
            }

            for p in moves(robot) {
                print_maze(p);
            }
        }

        break;
    }
}
fn solve_two(input: Vec<String>) {}

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
