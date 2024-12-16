use regex::Regex;
use std::{collections::HashSet, fs};

pub fn part_1() {
    let file_path = "src/day14/input.txt";
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let width = 101;
    let height = 103;
    let seconds = 100;

    let mut q: Vec<u32> = vec![0; 4];
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    for line in data.lines() {
        if let Some(caps) = re.captures(line) {
            let p = (
                caps[1].parse::<i32>().unwrap(),
                caps[2].parse::<i32>().unwrap(),
            );
            let v = (
                caps[3].parse::<i32>().unwrap(),
                caps[4].parse::<i32>().unwrap(),
            );

            let nx = (p.0 + seconds * v.0).rem_euclid(width);
            let ny = (p.1 + seconds * v.1).rem_euclid(height);

            if nx < 50 && ny < 51 {
                q[0] += 1;
            } else if nx > 50 && ny < 51 {
                q[1] += 1;
            } else if nx < 50 && ny > 51 {
                q[2] += 1;
            } else if nx > 50 && ny > 51 {
                q[3] += 1;
            }
        }
    }

    let answer = q[0] * q[1] * q[2] * q[3];
    println!("Answer for day 14 pt1 puzzle: {}", answer);
}

pub fn part_2() {
    let file_path = "src/day14/input.txt";
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let width = 101;
    let height = 103;

    let mut answer = 0;

    for seconds in 0..100000 {
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        for line in data.lines() {
            if let Some(caps) = re.captures(line) {
                let p = (
                    caps[1].parse::<i32>().unwrap(),
                    caps[2].parse::<i32>().unwrap(),
                );
                let v = (
                    caps[3].parse::<i32>().unwrap(),
                    caps[4].parse::<i32>().unwrap(),
                );

                let nx = (p.0 + seconds * v.0).rem_euclid(width);
                let ny = (p.1 + seconds * v.1).rem_euclid(height);

                seen.insert((nx, ny));
            }
        }

        if seen.len() == data.lines().count() {
            print_tree(&seen, width, height);
            answer = seconds;
            break;
        };
    }

    println!("Answer for day 14 pt1 puzzle: {}", answer);
}

pub fn print_tree(seen: &HashSet<(i32, i32)>, width: i32, height: i32) {
    for y in 0..height {
        let row: String = (0..width)
            .map(|x| if seen.contains(&(x, y)) { '#' } else { ' ' })
            .collect();
        println!("{}", row);
    }
}
