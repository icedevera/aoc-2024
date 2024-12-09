use std::fs;

pub fn part_1() {
    let file_path = "src/day7/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut correct_values_sum: u64 = 0;
    for line in data.lines() {
        let mut parts = line.split(':');

        let value = parts.next().unwrap().trim().parse::<u64>().unwrap();

        let num_string = parts.next().unwrap().trim();
        let nums: Vec<u64> = num_string
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();

        // check if we can produce the value from nums using only addition and multiplication
        if can_produce_value(&nums, value) {
            correct_values_sum += value;
        }
    }

    fn can_produce_value(nums: &[u64], target: u64) -> bool {
        fn dfs(nums: &[u64], current_value: u64, target: u64) -> bool {
            if nums.is_empty() {
                return current_value == target;
            }
            let next_num = nums[0];
            let remaining_nums = &nums[1..];

            // addition
            if dfs(remaining_nums, current_value + next_num, target) {
                return true;
            }
            // multiplication
            if dfs(remaining_nums, current_value * next_num, target) {
                return true;
            }

            false
        }

        if nums.is_empty() {
            return false;
        }

        dfs(&nums[1..], nums[0], target)
    }

    println!("Answer for day 7 pt1 puzzle: {}", correct_values_sum);
}

pub fn part_2() {
    let file_path = "src/day7/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut correct_values_sum: u64 = 0;
    for line in data.lines() {
        let mut parts = line.split(':');

        let value = parts.next().unwrap().trim().parse::<u64>().unwrap();

        let num_string = parts.next().unwrap().trim();
        let nums: Vec<u64> = num_string
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();

        // check if we can produce the value from nums using only addition multiplication or concatenation
        if can_produce_value(&nums, value) {
            correct_values_sum += value;
        }
    }

    fn can_produce_value(nums: &[u64], target: u64) -> bool {
        fn dfs(nums: &[u64], current_value: u64, target: u64) -> bool {
            if nums.is_empty() {
                return current_value == target;
            }
            let next_num = nums[0];
            let remaining_nums = &nums[1..];

            // addition
            if dfs(remaining_nums, current_value + next_num, target) {
                return true;
            }
            // multiplication
            if dfs(remaining_nums, current_value * next_num, target) {
                return true;
            }

            //  concatenation
            if let Some(concatenated_value) = concat_numbers(current_value, next_num) {
                if dfs(remaining_nums, concatenated_value, target) {
                    return true;
                }
            }

            false
        }

        if nums.is_empty() {
            return false;
        }

        dfs(&nums[1..], nums[0], target)
    }

    fn concat_numbers(a: u64, b: u64) -> Option<u64> {
        let concatenated = format!("{}{}", a, b);
        concatenated.parse::<u64>().ok()
    }

    println!("Answer for day 7 pt2 puzzle: {}", correct_values_sum);
}
