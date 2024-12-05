use regex::Regex;
use std::fs;

pub fn part_1() {
    let file_path = "src/day3/input.txt";

    // Open the file
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let functions = extract_mut_statements(&data);

    let mut sum: i32 = 0;
    for function in functions.iter() {
        let remove_mul = function.replace("mul", "");
        let remove_parentheses = remove_mul.replace("(", "").replace(")", "");
        let values: Vec<i32> = remove_parentheses
            .split(",")
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        let result = values[0] * values[1];

        sum += result;
    }

    println!("Answer for day 3 pt1 puzzle: {}", sum);
}

fn extract_mut_statements(input: &str) -> Vec<String> {
    // Define the regex pattern
    let pattern = r"mul\((-?\d+),(-?\d+)\)";
    let regex = Regex::new(pattern).unwrap();

    // Find all matches and collect them as strings
    regex
        .captures_iter(input)
        .map(|caps| caps[0].to_string())
        .collect()
}

pub fn part_2() {
    let file_path = "src/day3/input.txt";

    // Open the file
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let functions = extract_mut_with_conditional_statements(&data);

    let mut sum: i32 = 0;
    let mut is_do = true;
    for function in functions.iter() {
        if function == "do()" {
            is_do = true;
            continue;
        } else if function == "don't()" {
            is_do = false;
            continue;
        }

        if !is_do {
            continue;
        }

        let remove_mul = function.replace("mul", "");
        let remove_parentheses = remove_mul.replace("(", "").replace(")", "");
        let values: Vec<i32> = remove_parentheses
            .split(",")
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        let result = values[0] * values[1];

        sum += result;
    }

    println!("Answer for day 3 pt1 puzzle: {}", sum);
}

pub fn extract_mut_with_conditional_statements(input: &str) -> Vec<String> {
    // Define the regex pattern
    let pattern = r"mul\((-?\d+),(-?\d+)\)|do\(\)|don't\(\)";
    let regex = Regex::new(pattern).unwrap();

    // Find all matches and collect them as strings
    regex
        .find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .collect()
}
