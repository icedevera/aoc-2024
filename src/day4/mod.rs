use std::fs;

pub fn part_1() {
    let file_path = "src/day4/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let lines: Vec<&str> = data.trim().lines().collect();

    let height = lines.len();
    let width = lines.first().map_or(0, |line| line.len());

    let mut num_occurrances = 0;

    let letters = ['X', 'M', 'A', 'S'];

    let get_char_at = |x: usize, y: usize| -> Option<char> {
        if x < height && y < width {
            lines[x].chars().nth(y)
        } else {
            None
        }
    };

    let check_direction = |x: usize, y: usize, dx: isize, dy: isize| -> bool {
        for (k, &letter) in letters.iter().enumerate() {
            let new_x = x as isize + k as isize * dx;
            let new_y = y as isize + k as isize * dy;

            if new_x < 0 || new_y < 0 {
                return false;
            }

            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if get_char_at(new_x, new_y) != Some(letter) {
                return false;
            }
        }
        true
    };

    for x in 0..height {
        for y in 0..width {
            if get_char_at(x, y) == Some('X') {
                let directions = [
                    (-1, -1), // Northwest
                    (-1, 0),  // North
                    (-1, 1),  // Northeast
                    (0, 1),   // East
                    (1, 1),   // Southeast
                    (1, 0),   // South
                    (1, -1),  // Southwest
                    (0, -1),  // West
                ];

                for &(dx, dy) in &directions {
                    if check_direction(x, y, dx, dy) {
                        num_occurrances += 1;
                    }
                }
            }
        }
    }

    println!("Answer for day 4 pt1 puzzle: {}", num_occurrances);
}

pub fn part_2() {
    let file_path = "src/day4/input.txt";

    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let lines: Vec<&str> = data.trim().lines().collect();

    let height = lines.len();
    let width = lines.first().map_or(0, |line| line.len());

    let mut num_occurrances = 0;

    let get_char_at = |x: usize, y: usize| -> Option<char> {
        if x < height && y < width {
            lines[x].chars().nth(y)
        } else {
            None
        }
    };

    let check_x_mas = |x: usize, y: usize| -> bool {
        let ne_x = x as isize - 1;
        let ne_y = y as isize + 1;
        let sw_x = x as isize + 1;
        let sw_y = y as isize - 1;

        let nw_x = x as isize - 1;
        let nw_y = y as isize - 1;
        let se_x = x as isize + 1;
        let se_y = y as isize + 1;

        if ne_x < 0
            || ne_y < 0
            || sw_x < 0
            || sw_y < 0
            || nw_x < 0
            || nw_y < 0
            || se_x < 0
            || se_y < 0
        {
            return false;
        }

        let ne_char = get_char_at(ne_x as usize, ne_y as usize);
        let sw_char = get_char_at(sw_x as usize, sw_y as usize);
        let nw_char = get_char_at(nw_x as usize, nw_y as usize);
        let se_char = get_char_at(se_x as usize, se_y as usize);

        if (ne_char == Some('S') && sw_char == Some('M'))
            || (ne_char == Some('M') && sw_char == Some('S'))
        {
        } else {
            return false;
        }

        if (nw_char == Some('S') && se_char == Some('M'))
            || (nw_char == Some('M') && se_char == Some('S'))
        {
        } else {
            return false;
        }

        return true;
    };

    for x in 0..height {
        for y in 0..width {
            if get_char_at(x, y) == Some('A') {
                if check_x_mas(x, y) {
                    num_occurrances += 1;
                }
            }
        }
    }

    println!("Answer for day 4 pt2 puzzle: {}", num_occurrances);
}
