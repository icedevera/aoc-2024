use std::collections::HashMap;
use std::fs;

pub fn part_1() {
    let file_path = "src/day1/input.txt";

    // Open the file
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    // Vectors to hold column data
    let mut col1: Vec<i64> = Vec::new();
    let mut col2: Vec<i64> = Vec::new();

    // Process each line
    for line in data.lines() {
        let columns: Vec<&str> = line.split_whitespace().collect();

        if columns.len() == 2 {
            // Parse columns into integers and push into vectors
            if let (Ok(val1), Ok(val2)) = (columns[0].parse::<i64>(), columns[1].parse::<i64>()) {
                col1.push(val1);
                col2.push(val2);
            } else {
                eprintln!("Failed to parse line: {}", line);
            }
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    col1.sort();
    col2.sort();

    let mut sum: i64 = 0;
    for i in 0..col1.len() {
        let diff = col1[i] - col2[i];
        sum += diff.abs();
    }

    println!("Answer for day 1 pt1 puzzle: {}", sum);
}

pub fn part_2() {
    let file_path = "src/day1/input.txt";

    // Open the file
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    // Vectors to hold column data

    let mut col1_map: HashMap<i64, i64> = HashMap::new();
    let mut col2_map: HashMap<i64, i64> = HashMap::new();

    // Process each line
    for line in data.lines() {
        let columns: Vec<&str> = line.split_whitespace().collect();

        if columns.len() == 2 {
            // Parse columns into integers and push into vectors
            if let (Ok(val1), Ok(val2)) = (columns[0].parse::<i64>(), columns[1].parse::<i64>()) {
                if col1_map.contains_key(&val1) {
                    let count = col1_map.get(&val1).unwrap();
                    col1_map.insert(val1, count + 1);
                } else {
                    col1_map.insert(val1, 1);
                }

                if col2_map.contains_key(&val2) {
                    let count = col2_map.get(&val2).unwrap();
                    col2_map.insert(val2, count + 1);
                } else {
                    col2_map.insert(val2, 1);
                }
            } else {
                eprintln!("Failed to parse line: {}", line);
            }
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    let mut sum: i64 = 0;
    for (key, value) in col1_map.iter() {
        if col2_map.contains_key(key) {
            if col2_map.contains_key(key) {
                let count = col2_map.get(key).unwrap();
                let product = key * count;
                let repeat = product * value;
                sum += repeat;
            } else {
                sum += 0
            }
        }
    }

    println!("Answer for day 1 pt2 puzzle: {}", sum);
}
