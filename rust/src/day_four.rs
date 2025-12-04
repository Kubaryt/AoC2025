pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let part_one_result = part_one(&lines);
    println!("Part One Result: {}", part_one_result);
    let part_two_result = part_two(lines.iter().map(|s| s.to_string()).collect());
    println!("Part Two Result: {}", part_two_result);
}

fn part_one(lines: &[&str]) -> u32 {
    let mut accessible_paper = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '@' {
                let neighbours = count_neighbours(lines, y, x);
                if neighbours < 4 {
                    accessible_paper += 1;
                }
            }
        }
    }

    accessible_paper
}

fn part_two(mut lines: Vec<String>) -> u32 {
    let mut accessible_paper = 0;
    let mut new_paper_removed = false;
    loop {
        let mut new_lines = vec![];
        for (y, line) in lines.iter().enumerate() {
            let mut current_line = String::new();
            for (x, ch) in line.chars().enumerate() {
                if ch == '@' {
                    let neighbours = count_neighbours(
                        &lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                        y,
                        x,
                    );
                    if neighbours < 4 {
                        accessible_paper += 1;
                        new_paper_removed = true;
                        current_line.push('.');
                        continue;
                    }
                    current_line.push('@');
                    continue;
                }
                current_line.push(ch);
            }
            new_lines.push(current_line);
        }
        if !new_paper_removed {
            break;
        }
        new_paper_removed = false;
        lines = new_lines;
    }
    accessible_paper
}

fn count_neighbours(lines: &[&str], y: usize, x: usize) -> u32 {
    let mut neighbours = 0;
    for i in (y as i32) - 1..=(y as i32) + 1 {
        if i < 0 || i >= lines.len() as i32 {
            continue;
        }
        for j in (x as i32) - 1..=(x as i32) + 1 {
            if j < 0 || j >= lines[i as usize].len() as i32 {
                continue;
            }
            if i == y as i32 && j == x as i32 {
                continue;
            }
            if lines[i as usize].chars().nth(j as usize).unwrap() == '@' {
                neighbours += 1;
            }
        }
    }
    neighbours
}

mod test {
    #[test]
    fn test_with_test_input() {
        let input = vec![
            String::from("..@@.@@@@."),
            String::from("@@@.@.@.@@"),
            String::from("@@@@@.@.@@"),
            String::from("@.@@@@..@."),
            String::from("@@.@@@@.@@"),
            String::from(".@@@@@@@.@"),
            String::from(".@.@.@.@@@"),
            String::from("@.@@@.@@@@"),
            String::from(".@@@@@@@@."),
            String::from("@.@.@@@.@."),
        ];
        let result = super::part_one(&input.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
        assert_eq!(result, 13);
        let result = super::part_two(input);
        assert_eq!(result, 43);
    }
}
