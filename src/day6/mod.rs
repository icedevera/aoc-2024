use std::collections::{HashMap, HashSet};
use std::fs;

pub fn part_1() {
    let file_path = "src/day6/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let mut caret_position: Option<(usize, usize)> = None;
    let mut hash_positions: HashMap<(usize, usize), char> = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            match ch {
                '^' | '>' | '<' | 'v' => {
                    caret_position = Some((row_index, col_index));
                }
                '#' => {
                    hash_positions.insert((row_index, col_index), ch);
                }
                _ => {}
            }
        }
    }

    let (mut x, mut y) = caret_position.expect("Caret position not found in the grid.");
    let mut direction = grid[x][y];

    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    visited_positions.insert((x, y));

    let guard_move = |x: usize, y: usize, direction: char| -> (usize, usize, char) {
        let turn_right = |dir: char| -> char {
            match dir {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => dir,
            }
        };

        let (next_x, next_y) = match direction {
            '^' => (x.wrapping_sub(1), y),
            '>' => (x, y + 1),
            'v' => (x + 1, y),
            '<' => (x, y.wrapping_sub(1)),
            _ => (x, y),
        };

        if hash_positions.contains_key(&(next_x, next_y)) {
            let new_direction = turn_right(direction);
            (x, y, new_direction)
        } else {
            (next_x, next_y, direction)
        }
    };

    loop {
        let (new_x, new_y, new_direction) = guard_move(x, y, direction);

        if new_x >= grid.len() || new_y >= grid[0].len() {
            break;
        }

        x = new_x;
        y = new_y;
        direction = new_direction;

        visited_positions.insert((x, y));
    }

    println!("Answer for day 6 pt1 puzzle: {}", visited_positions.len());
}

pub fn part_2() {
    let file_path = "src/day6/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let mut caret_position: Option<(usize, usize)> = None;
    let mut hash_positions: HashMap<(usize, usize), char> = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            match ch {
                '^' | '>' | '<' | 'v' => {
                    caret_position = Some((row_index, col_index));
                }
                '#' => {
                    hash_positions.insert((row_index, col_index), ch);
                }
                _ => {}
            }
        }
    }

    let (start_x, start_y) = caret_position.expect("Caret position not found in the grid.");
    let mut direction = grid[start_x][start_y];

    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    visited_positions.insert((start_x, start_y));

    let mut x = start_x;
    let mut y = start_y;

    let guard_move = |x: usize,
                      y: usize,
                      direction: char,
                      hash_positions: &HashMap<(usize, usize), char>|
     -> (usize, usize, char) {
        let turn_right = |dir: char| -> char {
            match dir {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => dir,
            }
        };

        let (next_x, next_y) = match direction {
            '^' => (x.wrapping_sub(1), y),
            '>' => (x, y + 1),
            'v' => (x + 1, y),
            '<' => (x, y.wrapping_sub(1)),
            _ => (x, y),
        };

        if hash_positions.contains_key(&(next_x, next_y)) {
            let new_direction = turn_right(direction);
            (x, y, new_direction)
        } else {
            (next_x, next_y, direction)
        }
    };

    // do part_1 to get visited positions
    loop {
        let (new_x, new_y, new_direction) = guard_move(x, y, direction, &hash_positions);
        if new_x >= grid.len() || new_y >= grid[0].len() {
            break;
        }
        x = new_x;
        y = new_y;
        direction = new_direction;
        visited_positions.insert((x, y));
    }

    // brute force: try placing obstacle in all visited positions and see if it's a loop
    let mut num_obstacles = 0;
    for (vx, vy) in visited_positions.iter() {
        if (*vx, *vy) == (start_x, start_y) {
            continue;
        }

        // temp add obstacle
        hash_positions.insert((*vx, *vy), '#');

        // resimulate the guard's movement
        let mut temp_visited_states: HashSet<(usize, usize, char)> = HashSet::new();
        let mut temp_x = start_x;
        let mut temp_y = start_y;
        let mut temp_direction = grid[start_x][start_y];
        temp_visited_states.insert((temp_x, temp_y, temp_direction));

        let mut loop_detected = false;

        loop {
            let (new_x, new_y, new_direction) =
                guard_move(temp_x, temp_y, temp_direction, &hash_positions);

            if new_x >= grid.len() || new_y >= grid[0].len() {
                break;
            }

            temp_x = new_x;
            temp_y = new_y;
            temp_direction = new_direction;

            if !temp_visited_states.insert((temp_x, temp_y, temp_direction)) {
                loop_detected = true;
                break;
            }
        }

        if loop_detected {
            num_obstacles += 1;
        }

        hash_positions.remove(&(*vx, *vy));
    }

    println!("Answer for day 6 pt2 puzzle: {}", num_obstacles);
}
