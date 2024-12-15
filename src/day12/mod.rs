use std::collections::{HashMap, HashSet};
use std::fs;

pub fn part_1() {
    let file_path = "src/day12/input.txt";
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut total_price = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited[i][j] {
                let (area, perimeter) = flood_fill_1(&grid, &mut visited, i, j, grid[i][j]);
                total_price += area * perimeter;
            }
        }
    }

    println!("Answer for day 12 pt1 puzzle: {}", total_price);
}

fn flood_fill_1(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_row: usize,
    start_col: usize,
    region_type: char,
) -> (usize, usize) {
    let mut stack = vec![(start_row, start_col)];
    let mut area = 0;
    let mut perimeter = 0;

    while let Some((row, col)) = stack.pop() {
        if visited[row][col] {
            continue;
        }

        visited[row][col] = true;
        area += 1;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row < 0
                || new_row >= grid.len() as isize
                || new_col < 0
                || new_col >= grid[0].len() as isize
            {
                perimeter += 1;
            } else {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] == region_type && !visited[new_row][new_col] {
                    stack.push((new_row, new_col));
                } else if grid[new_row][new_col] != region_type {
                    perimeter += 1;
                }
            }
        }
    }

    (area, perimeter)
}

// help from https://www.youtube.com/watch?v=glNiVe_Rztg
type Point = (usize, usize);
pub fn part_2() {
    let file_path = "src/day12/input.txt";
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let n = grid.len();
    let m = grid[0].len();
    let mut cc = HashMap::new(); // Maps each point to its connected component ID
    let mut next_id = 0;

    // DFS to find connected components
    for i in 0..n {
        for j in 0..m {
            if !cc.contains_key(&(i, j)) {
                dfs(&grid, &mut cc, i, j, grid[i][j], next_id);
                next_id += 1;
            }
        }
    }

    // Group nodes by their connected component
    let mut ccs: HashMap<usize, HashSet<Point>> = HashMap::new();
    for (point, id) in &cc {
        ccs.entry(*id).or_insert_with(HashSet::new).insert(*point);
    }
    let mut total_price = 0;

    // Calculate area, perimeter, and corners for each connected component
    for nodes in ccs.values() {
        let area = nodes.len();
        let mut perimeter = HashSet::new();
        for &(x, y) in nodes {
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0
                    || ny < 0
                    || nx >= n as isize
                    || ny >= m as isize
                    || !nodes.contains(&(nx as usize, ny as usize))
                {
                    // Safely add to perimeter
                    let valid_nx = if nx >= 0 && nx < n as isize {
                        nx as usize
                    } else {
                        usize::MAX
                    };
                    let valid_ny = if ny >= 0 && ny < m as isize {
                        ny as usize
                    } else {
                        usize::MAX
                    };
                    perimeter.insert(((x, y), (valid_nx, valid_ny)));
                }
            }
        }

        let mut filtered_perimeter = HashSet::new();
        for &((x1, y1), (x2, y2)) in &perimeter {
            let mut keep = true;
            for &(dx, dy) in &[(1, 0), (0, 1)] {
                let p1n = ((x1 as isize + dx) as usize, (y1 as isize + dy) as usize);
                let p2n = ((x2 as isize + dx) as usize, (y2 as isize + dy) as usize);

                if perimeter.contains(&(p1n, p2n)) {
                    keep = false;
                }
            }
            if keep {
                filtered_perimeter.insert(((x1, y1), (x2, y2)));
            }
        }
        total_price += area * filtered_perimeter.len();
    }

    println!("Answer for day 12 pt2 puzzle: {}", total_price);
}

fn dfs(
    grid: &Vec<Vec<char>>,
    cc: &mut HashMap<Point, usize>,
    x: usize,
    y: usize,
    region_type: char,
    id: usize,
) {
    let n = grid.len();
    let m = grid[0].len();
    let mut stack = vec![(x, y)];

    while let Some((cx, cy)) = stack.pop() {
        if cc.contains_key(&(cx, cy)) {
            continue;
        }
        cc.insert((cx, cy), id);

        for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;

            if nx >= 0
                && ny >= 0
                && nx < n as isize
                && ny < m as isize
                && grid[nx as usize][ny as usize] == region_type
                && !cc.contains_key(&(nx as usize, ny as usize))
            {
                stack.push((nx as usize, ny as usize));
            }
        }
    }
}
