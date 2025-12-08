pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let part_one_result = part_one(&lines);
    println!("Part One: {}", part_one_result);
    let part_two_result = part_two(&lines);
    println!("Part Two: {}", part_two_result);
}

fn part_one(lines: &Vec<&str>) -> usize {
    let mut total = 0;
    let mut numbers: Vec<Vec<usize>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if i != lines.len() - 1 {
            let nums: Vec<usize> = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            numbers.push(nums);
        } else {
            operators = line
                .split_whitespace()
                .map(|x| x.chars().next().unwrap())
                .collect();
        }
    }
    for i in 0..numbers[0].len() {
        let mut local_result = 0;
        for number in &numbers {
            match operators[i] {
                '+' => {
                    local_result += number[i];
                }
                '*' => {
                    if local_result == 0 {
                        local_result = 1;
                    }
                    local_result *= number[i];
                }
                _ => {}
            }
        }
        total += local_result;
    }
    total
}

fn part_two(lines: &Vec<&str>) -> usize {
    let mut total = 0;
    let mut operators: Vec<char> = Vec::new();
    let mut column_spacing: Vec<usize> = Vec::new();
    let mut num_columns: Vec<Vec<String>> = Vec::new();
    let last_line = lines.last().unwrap();
    let lines = &lines[..lines.len() - 1];
    let mut column_start = 0;
    for (i, c) in last_line.chars().enumerate() {
        if c == ' ' {
            continue;
        } else {
            operators.push(c);
            if i != 0 {
                column_spacing.push(i - column_start - 1);
            }
            column_start = i;
        }
    }
    column_spacing.push(last_line.len() - column_start);
    for line in lines {
        let mut string_buffer = String::new();
        let mut current_column = 0;
        for char in line.chars() {
            let current_line_spacing = column_spacing[current_column];
            if string_buffer.len() < current_line_spacing {
                string_buffer.push(char);
            } else {
                if num_columns.len() <= current_column {
                    num_columns.push(Vec::new());
                }
                num_columns[current_column].push(string_buffer);
                string_buffer = String::new();
                current_column += 1;
            }
        }
        if !string_buffer.is_empty() {
            if num_columns.len() <= current_column {
                num_columns.push(Vec::new());
            }
            num_columns[current_column].push(string_buffer);
        }
    }

    for (i, column) in num_columns.iter().enumerate() {
        let mut local_result = 0;
        for j in 0..column_spacing[i] {
            let mut num_buffer = String::new();
            for num_str in column {
                num_buffer.push_str(&num_str[j..=j])
            }
            match operators[i] {
                '+' => {
                    local_result += num_buffer.trim().parse::<usize>().unwrap();
                }
                '*' => {
                    if local_result == 0 {
                        local_result = 1;
                    }
                    local_result *= num_buffer.trim().parse::<usize>().unwrap();
                }
                _ => {}
            }
        }
        total += local_result;
    }

    total
}

mod test {
    #[test]
    fn test_with_test_input() {
        let input = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];
        let result = super::part_one(&input);
        assert_eq!(result, 4277556);
        let result_two = super::part_two(&input);
        assert_eq!(result_two, 3263827);
    }
}
