use ndarray::array;
use ndarray_linalg::solve::Solve;
use regex::Regex;
use std::fs;

pub fn part_1() {
    let file_path = "src/day13/input.txt";
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let re = Regex::new(r"[+-]?\d+").unwrap();

    let mut result: u64 = 0;
    for block in data.trim().split("\n\n") {
        let lines: Vec<&str> = block.lines().collect();

        let button_a: Vec<u64> = re
            .captures_iter(lines[0])
            .map(|cap| cap[0].parse::<u64>().unwrap())
            .collect();
        let button_b: Vec<u64> = re
            .captures_iter(lines[1])
            .map(|cap| cap[0].parse::<u64>().unwrap())
            .collect();
        let prize: Vec<u64> = re
            .captures_iter(lines[2])
            .map(|cap| cap[0].parse::<u64>().unwrap())
            .collect();
        let mut min = u64::MAX;

        for i in 0..=100 {
            for j in 0..=100 {
                let nx: u64 = button_a[0] * i + button_b[0] * j;
                let ny: u64 = button_a[1] * i + button_b[1] * j;

                if prize[0] == nx && prize[1] == ny {
                    min = min.min(3 * i + j);
                }
            }
        }

        if min != u64::MAX {
            result += min;
        }
    }

    println!("Answer for day 13 pt1 puzzle: {}", result);
}

pub fn part_2() {
    let file_path = "src/day13/input.txt";
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let re = Regex::new(r"[+-]?\d+").unwrap();

    let mut result: u64 = 0;
    for block in data.trim().split("\n\n") {
        let lines: Vec<&str> = block.lines().collect();

        let button_a: Vec<u64> = re
            .captures_iter(lines[0])
            .map(|cap| cap[0].parse::<u64>().unwrap())
            .collect();
        let button_b: Vec<u64> = re
            .captures_iter(lines[1])
            .map(|cap| cap[0].parse::<u64>().unwrap())
            .collect();
        let prize: Vec<u64> = re
            .captures_iter(lines[2])
            .map(|cap| cap[0].parse::<u64>().unwrap())
            .collect();

        let prize = vec![prize[0] + 10000000000000, prize[1] + 10000000000000];

        // systems of linear equations
        let a_matrix = array![
            [button_a[0] as f64, button_b[0] as f64],
            [button_a[1] as f64, button_b[1] as f64]
        ];
        let p_vector = array![prize[0] as f64, prize[1] as f64];

        let eq_result = a_matrix.solve(&p_vector).expect("Matrix invalid");

        let press_a = eq_result[0].round() as u64;
        let press_b = eq_result[1].round() as u64;

        if press_a * button_a[0] + press_b * button_b[0] == prize[0]
            && press_a * button_a[1] + press_b * button_b[1] == prize[1]
        {
            result += 3 * press_a + press_b;
        }
    }

    println!("Answer for day 13 pt1 puzzle: {}", result);
}
