use std::collections::HashMap;

pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let part_one_result = part_one(&lines);
    println!("Part One Result: {}", part_one_result);
    let part_two_result = part_two(&lines);
    println!("Part Two Result: {}", part_two_result);
}

fn part_one(lines: &Vec<&str>) -> usize {
    let mut start_position: (usize, usize) = (0, 0);
    let mut manifold_positions: Vec<(usize, usize)> = Vec::new();
    for line in lines {
        if let Some(col) = line.find('S') {
            start_position = (lines.iter().position(|&l| &l == line).unwrap(), col);
        }
        for (col, ch) in line.chars().enumerate() {
            if ch == '^' {
                manifold_positions.push((lines.iter().position(|&l| &l == line).unwrap(), col));
            }
        }
    }
    let mut beams: Vec<(usize, usize)> = vec![(start_position.0 + 1, start_position.1)];
    let mut visited_positions: Vec<(usize, usize)> = vec![(start_position.0 + 1, start_position.1)];
    let mut split_counter = 0;
    while !beams.is_empty() {
        for i in 0..beams.len() {
            if visited_positions.contains(&(beams[i].0 + 1, beams[i].1)) {
                beams.remove(i);
                break;
            }
            beams[i].0 += 1;
            let coordinates = beams[i];
            if manifold_positions.contains(&coordinates) {
                let new_left = (coordinates.0, coordinates.1 - 1);
                let new_right = (coordinates.0, coordinates.1 + 1);
                if !beams.contains(&new_left) && !visited_positions.contains(&new_left) {
                    beams.push(new_left);
                    visited_positions.push(new_left);
                }
                if !beams.contains(&new_right) && !visited_positions.contains(&new_right) {
                    beams.push(new_right);
                    visited_positions.push(new_right);
                }
                beams.remove(i);
                split_counter += 1;
                break;
            }
            visited_positions.push(coordinates);
            if coordinates.0 == lines.len() - 1 {
                beams.remove(i);
                break;
            }
        }
        beams.sort();
    }

    split_counter
}

fn part_two(lines: &Vec<&str>) -> usize {
    let number_of_rows = lines.len();
    let number_of_cols = lines[0].len();
    let start_col = lines[0].find('S').unwrap();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    recursive_simulation(
        1,
        start_col,
        number_of_rows,
        number_of_cols,
        lines,
        &mut cache,
    )
}

fn recursive_simulation(
    row: usize,
    column: usize,
    number_of_rows: usize,
    number_of_cols: usize,
    lines: &Vec<&str>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if row >= number_of_rows || column >= number_of_cols {
        return 1;
    };

    if cache.contains_key(&(row, column)) {
        return *cache.get(&(row, column)).unwrap();
    };

    let total_paths;
    let current_cell = lines[row].chars().nth(column).unwrap();
    if current_cell == '^' {
        let mut left_path = 0;
        if column > 0 {
            left_path = recursive_simulation(
                row + 1,
                column - 1,
                number_of_rows,
                number_of_cols,
                lines,
                cache,
            );
        }
        let right_path = recursive_simulation(
            row + 1,
            column + 1,
            number_of_rows,
            number_of_cols,
            lines,
            cache,
        );
        total_paths = left_path + right_path;
    } else if current_cell == '.' {
        total_paths = recursive_simulation(
            row + 1,
            column,
            number_of_rows,
            number_of_cols,
            lines,
            cache,
        );
    } else {
        total_paths = 0;
    };

    cache.insert((row, column), total_paths);
    total_paths
}

mod test {
    #[test]
    fn test_with_test_input() {
        let input = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];
        let result = super::part_one(&input);
        assert_eq!(result, 21);
        let part_two_result = super::part_two(&input);
        assert_eq!(part_two_result, 40);
    }
}
