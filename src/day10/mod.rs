use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn part_1() {
    let file_path = "src/day10/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let grid: Vec<Vec<u8>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    fn bfs(grid: &Vec<Vec<u8>>, start_x: usize, start_y: usize) -> usize {
        let mut queue = VecDeque::new();
        let mut reachable_nines = HashSet::new();

        queue.push_back((start_x, start_y, 0));

        while let Some((x, y, height)) = queue.pop_front() {
            if grid[x][y] == 9 {
                reachable_nines.insert((x, y));
                continue;
            }

            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dx, dy) in directions.iter() {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                if new_x >= 0 && new_y >= 0 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;

                    if new_x < grid.len() && new_y < grid[0].len() {
                        if grid[new_x][new_y] == height + 1 {
                            queue.push_back((new_x, new_y, height + 1));
                        }
                    }
                }
            }
        }

        reachable_nines.len()
    }

    let mut total_score = 0;

    for row_index in 0..rows {
        for col_index in 0..cols {
            if grid[row_index][col_index] == 0 {
                total_score += bfs(&grid, row_index, col_index);
            }
        }
    }

    println!("Answer for day 10 part 1 puzzle: {}", total_score);
}

pub fn part_2() {
    let file_path = "src/day10/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let grid: Vec<Vec<u8>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    fn dfs(grid: &Vec<Vec<u8>>, x: usize, y: usize, target: u8) -> i32 {
        if x >= grid.len() || y >= grid[0].len() || grid[x][y] != target {
            return 0;
        }

        if target == 9 {
            return 1;
        }

        let next = target + 1;

        let mut trailheads = 0;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            if new_x < grid.len() && new_y < grid[0].len() {
                trailheads += dfs(grid, new_x, new_y, next);
            }
        }

        trailheads
    }

    let mut trailheads = 0;
    for row_index in 0..rows {
        for col_index in 0..cols {
            if grid[row_index][col_index] == 0 {
                trailheads += dfs(&grid, row_index, col_index, 0);
            }
        }
    }

    println!("Answer for day 10 pt2 puzzle: {}", trailheads);
}
