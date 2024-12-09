use std::collections::{HashMap, HashSet};
use std::fs;

pub fn part_1() {
    let file_path = "src/day8/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (x, line) in data.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c != '.' {
                antenna_map.entry(c).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    let grid_height = data.lines().count();
    let grid_width = data.lines().next().unwrap_or("").len();

    // calc antinodes
    let mut antinodes_set: HashSet<(usize, usize)> = HashSet::new();
    for (_freq, freq_positions) in antenna_map.iter() {
        let n = freq_positions.len();

        for i in 0..n {
            for j in i + 1..n {
                let (x1, y1) = freq_positions[i];
                let (x2, y2) = freq_positions[j];

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                let antinode1 = ((x1 as isize - dx) as usize, (y1 as isize - dy) as usize);

                let antinode2 = ((x2 as isize + dx) as usize, (y2 as isize + dy) as usize);

                if antinode1.0 < grid_height && antinode1.1 < grid_width {
                    antinodes_set.insert(antinode1);
                }
                if antinode2.0 < grid_height && antinode2.1 < grid_width {
                    antinodes_set.insert(antinode2);
                }
            }
        }
    }

    println!("Answer for day 8 pt1 puzzle: {}", antinodes_set.len());
}

pub fn part_2() {
    let file_path = "src/day8/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (x, line) in data.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c != '.' {
                antenna_map.entry(c).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    let grid_height = data.lines().count();
    let grid_width = data.lines().next().unwrap_or("").len();

    // calc antinodes
    let mut antinodes_set: HashSet<(usize, usize)> = HashSet::new();
    for (_freq, freq_positions) in antenna_map.iter() {
        let n = freq_positions.len();

        if n > 1 {
            // all positions are antinodes
            for &position in freq_positions {
                antinodes_set.insert(position);
            }
        }

        for i in 0..n {
            for j in i + 1..n {
                let (x1, y1) = freq_positions[i];
                let (x2, y2) = freq_positions[j];

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                let mut next_antinode_1 =
                    ((x1 as isize - dx) as usize, (y1 as isize - dy) as usize);

                let mut next_antinode_2 =
                    ((x2 as isize + dx) as usize, (y2 as isize + dy) as usize);

                while next_antinode_1.0 < grid_height && next_antinode_1.1 < grid_width {
                    antinodes_set.insert(next_antinode_1);
                    let (x, y) = next_antinode_1;
                    next_antinode_1 = ((x as isize - dx) as usize, (y as isize - dy) as usize);
                }

                while next_antinode_2.0 < grid_height && next_antinode_2.1 < grid_width {
                    antinodes_set.insert(next_antinode_2);
                    let (x, y) = next_antinode_2;
                    next_antinode_2 = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                }
            }
        }
    }

    println!("Answer for day 8 pt2 puzzle: {}", antinodes_set.len());
}
