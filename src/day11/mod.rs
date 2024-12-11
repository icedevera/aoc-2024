use std::collections::HashMap;
use std::fs;

pub fn part_1() {
    let file_path = "src/day11/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut stones: Vec<u64> = data
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Failed to parse number")) // Parse each part into u64
        .collect();

    for _ in 0..25 {
        let mut new_stones: Vec<u64> = vec![];

        let mut i = 0;
        while i < stones.len() {
            let num = stones[i];

            if num == 0 {
                new_stones.push(1);
            } else if num.to_string().len() % 2 == 0 {
                let num_str = num.to_string();

                let mid = num_str.len() / 2;

                let left: u64 = num_str[..mid].parse().unwrap();
                let right: u64 = num_str[mid..].parse().unwrap();

                if num_str == "253000" {
                    println!("Num: {}, Left: {}, Right: {}", num, left, right);
                }

                new_stones.push(left);
                new_stones.push(right);
            } else {
                new_stones.push(num * 2024)
            }

            i += 1;
        }

        stones = new_stones;
    }

    println!("Answer for day 11 pt1 puzzle: {}", stones.len());
}

pub fn part_2() {
    let file_path = "src/day11/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let stones: Vec<u64> = data
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Failed to parse number"))
        .collect();

    let mut memo: HashMap<(u64, u64), u64> = HashMap::new();

    fn transform_stone(num: u64) -> Vec<u64> {
        if num == 0 {
            vec![1]
        } else if num.to_string().len() % 2 == 0 {
            let num_str = num.to_string();
            let mid = num_str.len() / 2;

            let left: u64 = num_str[..mid].parse().unwrap();
            let right: u64 = num_str[mid..].parse().unwrap();

            vec![left, right]
        } else {
            vec![num * 2024]
        }
    }

    fn dfs(nums: Vec<u64>, depth: u64, max_depth: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
        if depth == max_depth {
            return nums.len() as u64;
        }

        let mut total_count = 0;

        for num in nums {
            if let Some(&cached_result) = memo.get(&(num, max_depth - depth)) {
                total_count += cached_result;
                continue;
            }

            let transformed = transform_stone(num);
            let count = dfs(transformed, depth + 1, max_depth, memo);

            memo.insert((num, max_depth - depth), count);
            total_count += count;
        }

        total_count
    }

    let max_depth = 75;
    let total_stones = dfs(stones, 0, max_depth, &mut memo);

    println!("Answer for day 11 pt2 puzzle: {}", total_stones);
}

pub fn part_2_optimized() {
    let file_path = "src/day11/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let stones: Vec<u64> = data
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Failed to parse number"))
        .collect();

    fn transform_stone(num: u64, memo: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
        if let Some(result) = memo.get(&num) {
            return result.clone();
        }

        let result = if num == 0 {
            vec![1]
        } else if num.to_string().len() % 2 == 0 {
            let num_str = num.to_string();
            let mid = num_str.len() / 2;

            let left: u64 = num_str[..mid].parse().unwrap();
            let right: u64 = num_str[mid..].parse().unwrap();

            vec![left, right]
        } else {
            vec![num * 2024]
        };

        memo.insert(num, result.clone());
        result
    }

    fn blink_n_times(nums: Vec<u64>, n: usize) -> u64 {
        let mut counts: HashMap<u64, u64> = HashMap::new();
        let mut memo: HashMap<u64, Vec<u64>> = HashMap::new();

        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        for _ in 0..n {
            let mut new_counts: HashMap<u64, u64> = HashMap::new();

            for (&num, &count) in &counts {
                for new_num in transform_stone(num, &mut memo) {
                    *new_counts.entry(new_num).or_insert(0) += count;
                }
            }

            counts = new_counts;
        }

        counts.values().sum()
    }

    let result = blink_n_times(stones, 75);

    println!("Answer for day 11 pt2 puzzle: {}", result);
}
