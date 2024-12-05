use std::collections::HashSet;
use std::fs;

pub fn part_1() {
    let file_path = "src/day5/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut xy_pairs = Vec::new();
    let mut number_lists = Vec::new();
    let mut is_number_list = false;

    for line in data.lines() {
        if line.trim().is_empty() {
            is_number_list = true;
            continue;
        }

        if is_number_list {
            let numbers: Vec<i32> = line
                .split(',')
                .map(|n| n.trim().parse().expect("Invalid number"))
                .collect();
            number_lists.push(numbers);
        } else {
            let pair: (i32, i32) = {
                let mut parts = line.split('|');
                (
                    parts.next().unwrap().parse().expect("Invalid x value"),
                    parts.next().unwrap().parse().expect("Invalid y value"),
                )
            };
            xy_pairs.push(pair);
        }
    }

    let xy_set: HashSet<(i32, i32)> = xy_pairs.into_iter().collect();

    let check_condition = |number_list: &Vec<i32>| -> bool {
        for &(num_before, num_after) in &xy_set {
            let pos_before = number_list.iter().position(|&n| n == num_before);
            let pos_after = number_list.iter().position(|&n| n == num_after);

            if let (Some(pos_b), Some(pos_a)) = (pos_before, pos_after) {
                if pos_b > pos_a {
                    return false;
                }
            }
        }
        true
    };

    let mut middle_sum = 0;

    for num_list in number_lists.iter() {
        if check_condition(num_list) {
            let middle_index = num_list.len() / 2;
            let middle_value = num_list[middle_index];
            middle_sum += middle_value;
        }
    }

    println!("Answer for day 5 pt1 puzzle: {}", middle_sum);
}

pub fn part_2() {
    let file_path = "src/day5/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut xy_pairs = Vec::new();
    let mut number_lists = Vec::new();
    let mut is_number_list = false;

    for line in data.lines() {
        if line.trim().is_empty() {
            is_number_list = true;
            continue;
        }

        if is_number_list {
            let numbers: Vec<i32> = line
                .split(',')
                .map(|n| n.trim().parse().expect("Invalid number"))
                .collect();
            number_lists.push(numbers);
        } else {
            let pair: (i32, i32) = {
                let mut parts = line.split('|');
                (
                    parts.next().unwrap().parse().expect("Invalid x value"),
                    parts.next().unwrap().parse().expect("Invalid y value"),
                )
            };
            xy_pairs.push(pair);
        }
    }

    let xy_set: HashSet<(i32, i32)> = xy_pairs.into_iter().collect();

    let check_condition = |number_list: &Vec<i32>| -> bool {
        for &(num_before, num_after) in &xy_set {
            let pos_before = number_list.iter().position(|&n| n == num_before);
            let pos_after = number_list.iter().position(|&n| n == num_after);

            if let (Some(pos_b), Some(pos_a)) = (pos_before, pos_after) {
                if pos_b > pos_a {
                    return false;
                }
            }
        }
        true
    };

    let fix_num_list = |number_list: &Vec<i32>| -> Vec<i32> {
        let mut new_list = number_list.clone();

        while !check_condition(&new_list) {
            for &(num_before, num_after) in &xy_set {
                let pos_before = new_list.iter().position(|&n| n == num_before);
                let pos_after = new_list.iter().position(|&n| n == num_after);

                if let (Some(pos_b), Some(pos_a)) = (pos_before, pos_after) {
                    if pos_b > pos_a {
                        new_list.swap(pos_b, pos_a);
                    }
                }
            }
        }

        new_list
    };

    let mut middle_sum = 0;

    for num_list in number_lists.iter() {
        if !check_condition(num_list) {
            let new_list = fix_num_list(num_list);

            let middle_index = new_list.len() / 2;
            let middle_value = new_list[middle_index];
            middle_sum += middle_value;
        }
    }

    println!("Answer for day 5 pt2 puzzle: {}", middle_sum);
}
