use std::fs;

pub fn part_1() {
    let file_path = "src/day2/input.txt";

    // Open the file
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut safe_count = 0;
    for line in data.lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        let is_increasing = values[0] < values[1];
        let mut is_safe = true;
        for i in 0..values.len() {
            if i == values.len() - 1 {
                break;
            }

            let val = values[i];
            let next = values[i + 1];

            if val == next || (val - next).abs() > 3 {
                is_safe = false;
                break;
            }

            if val < next {
                if !is_increasing {
                    is_safe = false;
                    break;
                }
            } else {
                if is_increasing == true {
                    is_safe = false;
                    break;
                }
            }

            is_safe = true;
        }

        if is_safe {
            safe_count += 1;
        }
    }

    println!("Answer for day 2 pt1 puzzle: {}", safe_count);
}

pub fn part_2() {
    let file_path = "src/day2/input.txt";

    // Open the file
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut safe_count = 0;
    for line in data.lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if is_safe(&values) || can_be_made_safe(&values) {
            safe_count += 1;
        }
    }

    println!("Answer for day 2 pt2 puzzle: {}", safe_count);
}

fn is_safe(values: &[i32]) -> bool {
    if values.len() < 2 {
        return true;
    }

    let is_increasing = values[0] < values[1];
    for i in 0..values.len() - 1 {
        if !is_safe_check(values[i], values[i + 1], is_increasing) {
            return false;
        }
    }
    true
}

fn can_be_made_safe(values: &[i32]) -> bool {
    for i in 0..values.len() {
        // Create a new sequence with the current element removed
        let mut new_values = values.to_vec();
        new_values.remove(i);
        if is_safe(&new_values) {
            return true;
        }
    }
    false
}

fn is_safe_check(val: i32, next: i32, is_increasing: bool) -> bool {
    if val == next || (val - next).abs() > 3 {
        return false;
    }

    if val < next {
        if !is_increasing {
            return false;
        }
    } else {
        if is_increasing {
            return false;
        }
    }

    return true;
}

// pub fn part_2() {
//     let file_path = "src/day2_input.txt";

//     // Open the file
//     let data = fs::read_to_string(file_path).expect("Unable to read file");

//     let mut safe_count = 0;
//     for line in data.lines() {
//         let values: Vec<i32> = line
//             .split_whitespace()
//             .filter_map(|num| num.parse::<i32>().ok())
//             .collect();

//         let mut is_increasing = values[0] < values[1];
//         let mut is_safe = true;
//         let mut is_problem_dampener_used = false;
//         for i in 0..values.len() {
//             if i == values.len() - 1 {
//                 break;
//             }

//             let val = values[i];
//             let next = values[i + 1];

//             let current = is_safe_check(val, next, is_increasing);

//             if !current {
//                 if is_problem_dampener_used {
//                     is_safe = false;
//                     break;
//                 }

//                 if i == 0 {
//                     // attempt to use problem dampener
//                     is_problem_dampener_used = true;
//                     is_increasing = values[1] < values[2];
//                     continue;
//                 } else {
//                     // check if removing current will make it safe
//                     if i == 1 {
//                         is_increasing = values[i - 1] < next;
//                     }
//                     let remove_current = is_safe_check(values[i - 1], next, is_increasing);
//                     if remove_current {
//                         is_problem_dampener_used = true;
//                     } else {
//                         is_safe = false;
//                         break;
//                     }
//                 }
//             }

//             is_safe = true;
//         }

//         if is_safe {
//             safe_count += 1;
//         }
//     }

//     println!("Answer for day 2 pt2 puzzle: {}", safe_count);
// }
