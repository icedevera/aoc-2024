use std::fs;

pub fn part_1() {
    let file_path = "src/day9/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut file_vec: Vec<i64> = Vec::new();
    let mut cur_id = 0;
    for (index, ch) in data.chars().enumerate() {
        if let Some(num) = ch.to_digit(10) {
            if index % 2 == 0 {
                file_vec.extend(vec![cur_id as i64; num as usize]);
                cur_id += 1;
            } else {
                file_vec.extend(vec![-1; num as usize]);
            }
        }
    }

    if let Some(mut free_space_index) = file_vec.iter().position(|&x| x == -1) {
        while let Some(last_digit_index) = file_vec.iter().rposition(|&x| x != -1) {
            if last_digit_index < free_space_index {
                break;
            }

            file_vec[free_space_index] = file_vec[last_digit_index];
            file_vec[last_digit_index] = -1;

            if let Some(next_free_space_index) = file_vec.iter().position(|&x| x == -1) {
                free_space_index = next_free_space_index;
            } else {
                break;
            }
        }
    }

    // Calculate the checksum
    let checksum: i64 = file_vec
        .iter()
        .enumerate()
        .filter(|(_, &id)| id != -1)
        .map(|(index, &id)| index as i64 * id)
        .sum();

    println!("Answer for day 9 pt1 puzzle: {}", checksum);
}

pub fn part_2() {
    let file_path = "src/day9/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut file_vec: Vec<i64> = Vec::new();
    let mut cur_id = 0;

    // Parse the input into file_vec
    for (index, ch) in data.chars().enumerate() {
        if let Some(num) = ch.to_digit(10) {
            if index % 2 == 0 {
                file_vec.extend(vec![cur_id as i64; num as usize]);
                cur_id += 1;
            } else {
                file_vec.extend(vec![-1; num as usize]);
            }
        }
    }

    // Process files in decreasing order of file ID
    for file_id in (0..cur_id).rev() {
        let file_indices: Vec<usize> = file_vec
            .iter()
            .enumerate()
            .filter_map(|(index, &block)| {
                if block == file_id as i64 {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        if file_indices.is_empty() {
            continue;
        }

        let file_length = file_indices.len();

        let mut free_space_start = None;
        let mut free_space_length = 0;

        for (index, &block) in file_vec.iter().enumerate() {
            if index >= file_indices[0] {
                break;
            }

            if block == -1 {
                if free_space_start.is_none() {
                    free_space_start = Some(index);
                }
                free_space_length += 1;

                if free_space_length == file_length {
                    break;
                }
            } else {
                free_space_start = None;
                free_space_length = 0;
            }
        }

        if let Some(start) = free_space_start {
            if free_space_length >= file_length {
                for (i, &file_block_index) in file_indices.iter().enumerate() {
                    file_vec[start + i] = file_id as i64;
                    file_vec[file_block_index] = -1;
                }
            }
        }
    }

    // Calculate the checksum
    let checksum: i64 = file_vec
        .iter()
        .enumerate()
        .filter(|(_, &block)| block != -1)
        .map(|(index, &block)| index as i64 * block)
        .sum();

    println!("Answer for day 9 part 2 puzzle: {}", checksum);
}
