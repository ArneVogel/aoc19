use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::env;
use std::hash::{Hash, Hasher};
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

fn robot_pos(maze: Vec<String>) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    for (i, line) in maze.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                output.push((i, j));
            }
        }
    }
    return output;
}

fn num_keys(maze: Vec<String>) -> i64 {
    let mut sum = 0;
    for (i, line) in maze.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '@' && c != '#' && c != '.' {
                sum += 1;
            }
        }
    }
    sum
}
fn moves_with_pos(maze: Vec<String>) -> Vec<((usize, usize), Vec<String>)> {
    let mut output: Vec<((usize, usize), Vec<String>)> = Vec::new();
    let positions = robot_pos(maze.clone());
    let keys = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    for pos in positions {
        /*
        if !reachable_key(maze.clone(), pos.clone()) {
            continue;
        }
        */
        for i in -1..2 {
            for j in -1..2 {
                if (i != 0 && j != 0) || (i == 0 && j == 0) {
                    continue;
                }
                let tx = pos.0 as i64 + i;
                let ty = pos.1 as i64 + j;
                let x = tx as usize;
                let y = ty as usize;

                if x >= 0 && y >= 0 && x < maze.len() && y < maze[0].len() {
                    if maze[x].chars().nth(y).unwrap() == '.' {
                        let mut clone = maze.clone();
                        clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
                        clone[x].replace_range(y..y + 1, "@");
                        output.push((pos.clone(), clone));
                    } else if keys.contains(&maze[x].chars().nth(y).unwrap()) {
                        let mut clone = maze.clone();
                        clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
                        let c = clone[x]
                            .chars()
                            .nth(y)
                            .unwrap()
                            .to_uppercase()
                            .to_string()
                            .chars()
                            .nth(0)
                            .unwrap();
                        for (i, line) in clone.clone().iter().enumerate() {
                            clone[i] = line.replace(c, ".");
                        }
                        clone[x].replace_range(y..y + 1, "@");
                        output.push((pos.clone(), clone));
                    }
                }
            }
        }
    }

    return output;
}

fn moves(maze: Vec<String>) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();
    let positions = robot_pos(maze.clone());
    let keys = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    for pos in positions {
        /*
        if !reachable_key(maze.clone(), pos.clone()) {
            continue;
        }
        */
        for i in -1..2 {
            for j in -1..2 {
                if (i != 0 && j != 0) || (i == 0 && j == 0) {
                    continue;
                }
                let tx = pos.0 as i64 + i;
                let ty = pos.1 as i64 + j;
                let x = tx as usize;
                let y = ty as usize;

                if x >= 0 && y >= 0 && x < maze.len() && y < maze[0].len() {
                    if maze[x].chars().nth(y).unwrap() == '.' {
                        let mut clone = maze.clone();
                        clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
                        clone[x].replace_range(y..y + 1, "@");
                        output.push(clone);
                    } else if keys.contains(&maze[x].chars().nth(y).unwrap()) {
                        let mut clone = maze.clone();
                        clone[pos.0].replace_range(pos.1..pos.1 + 1, ".");
                        let c = clone[x]
                            .chars()
                            .nth(y)
                            .unwrap()
                            .to_uppercase()
                            .to_string()
                            .chars()
                            .nth(0)
                            .unwrap();
                        for (i, line) in clone.clone().iter().enumerate() {
                            clone[i] = line.replace(c, ".");
                        }
                        clone[x].replace_range(y..y + 1, "@");
                        output.push(clone);
                    }
                }
            }
        }
    }

    return output;
}

fn reachable_key(maze: Vec<String>, pos: (usize, usize)) -> bool {
    let mut my_maze = maze.clone();
    my_maze[pos.0].replace_range(pos.1..pos.1 + 1, "1");
    let keys = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    loop {
        let mut change = false;
        for (i, line) in my_maze.clone().iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                for z in -1..2 {
                    for k in -1..2 {
                        if (z != 0 && k != 0) || (z == 0 && k == 0) {
                            continue;
                        }
                        let tx = i as i64 + z;
                        let ty = j as i64 + k;
                        let x = tx as usize;
                        let y = ty as usize;
                        if x < 0 || y < 0 || x > maze.len() - 1 || y > maze.len() - 1 {
                            continue;
                        }
                        if c == '1' && my_maze[x].chars().nth(y).unwrap() == '.' {
                            my_maze[x].replace_range(y..y + 1, "1");
                        }
                        if keys.contains(&my_maze[x].chars().nth(y).unwrap()) {
                            return true;
                        }
                    }
                }
            }
        }

        if !change {
            break;
        }
    }
    return false;
}

fn print_maze(maze: Vec<String>) {
    for line in maze {
        println!("{}", line);
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn solve_one(input: Vec<String>) {
    let mut robots: Vec<Vec<String>> = Vec::new();
    let mut all: HashSet<u64> = HashSet::new();
    robots.push(input.clone());
    let mut sum = 0;
    let mut min_keys = num_keys(input.clone());

    all.insert(calculate_hash(&input.clone()));
    loop {
        sum += 1;
        dbg!(sum);
        robots.sort();
        robots.dedup();
        let mut old_robots = robots.clone();
        robots.clear();
        for robot in old_robots {
            if solved(robot.clone()) {
                dbg!(sum - 1);
                return;
            }
            //print_maze(robot.clone());

            for p in moves(robot) {
                //print_maze(p.clone());
                /*
                if num_keys(p.clone()) >= min_keys + 30 {
                    continue;
                }
                */
                if !all.insert(calculate_hash(&p.clone())) {
                    continue;
                }
                min_keys = cmp::min(min_keys, num_keys(p.clone()));
                robots.push(p);
            }
        }
        /*
        let copy = all.clone();
        all.drain();
        for a in copy.clone().iter().filter(|x| num_keys(x.to_vec()) >= (min_keys+2)) {
            all.insert(a.to_vec());
        }
        */
    }
}

fn split_four(input: Vec<String>) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();
    output.push(Vec::new());
    output.push(Vec::new());
    output.push(Vec::new());
    output.push(Vec::new());
    for (i, line) in input.iter().enumerate() {
        if i <= input.len()/2 {
            output[0].push(line[0..line.len() / 2].to_string());
            output[1].push(line[line.len() / 2..line.len()].to_string());
        } else {
            output[2].push(line[0..line.len() / 2].to_string());
            output[3].push(line[line.len() / 2..line.len()].to_string());
        }
    }

    return output;
}

fn which_quadrant(pos: (usize, usize), input: Vec<String>) -> usize {
    if pos.0 <= input.len() / 2 && pos.1 <= input[0].len() / 2 {
        return 0;
    }

    if pos.0 <= input.len() / 2 && pos.1 > input[0].len() / 2 {
        return 1;
    }
    if pos.1 <= input[0].len() / 2 {
        return 2;
    }

    return 3;
}

fn solve_two(input: Vec<String>) {
    let mut robots: Vec<Vec<String>> = Vec::new();
    let mut all: HashSet<u64> = HashSet::new();
    robots.push(input.clone());
    let mut sum = 0;
    let mut min_keys = num_keys(input.clone());

    for s in split_four(input.clone()) {
        print_maze(s.clone());
        println!("hi");
        all.insert(calculate_hash(&s));
    }
    loop {
        sum += 1;
        dbg!(sum,all.len());
        robots.sort();
        robots.dedup();
        let mut old_robots = robots.clone();
        robots.clear();
        for robot in old_robots {
            if solved(robot.clone()) {
                dbg!(sum - 1);
                return;
            }
            //print_maze(robot.clone());

            for (pos, p) in moves_with_pos(robot) {
                if !all.insert(calculate_hash(
                    &split_four(p.clone())[which_quadrant(pos, input.clone())],
                )) {
                    continue;
                }
                robots.push(p);
            }
        }
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
