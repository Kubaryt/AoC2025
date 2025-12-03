pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let part_one_result = part_one(&lines);
    println!("Part One Result: {}", part_one_result);
    let part_two_result = part_two(&lines);
    println!("Part Two Result: {}", part_two_result);
}

fn part_one(lines: &Vec<&str>) -> u32 {
    let mut joltage_sum = 0;
    for line in lines {
        let highest_two_digit_number = calculate_number(line, 2);
        joltage_sum += highest_two_digit_number as u32;
    }

    joltage_sum
}

fn part_two(lines: &Vec<&str>) -> u64 {
    let mut joltage_sum = 0;
    for line in lines {
        let highest_twelve_digit_number = calculate_number(line, 12);
        joltage_sum += highest_twelve_digit_number;
    }

    joltage_sum
}

fn calculate_number(mut line: &str, length: u8) -> u64 {
    let mut highest_number = 0u64;
    for i in (1..length + 1).rev() {
        let mut highest_single_digit = 0;
        let mut highest_digit_index = 0;
        for (index, digit) in line[..line.len() - (i as usize) + 1].chars().enumerate() {
            let digit_value = digit.to_digit(10).unwrap() as u8;
            if digit_value > highest_single_digit {
                highest_single_digit = digit_value;
                highest_digit_index = index;
                if digit_value == 9 {
                    break;
                }
            }
        }
        highest_number += (highest_single_digit as u64) * 10u64.pow((i - 1) as u32);
        line = &line[highest_digit_index + 1..];
    }
    highest_number
}

mod test {
    #[test]
    fn test_part_one() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let result = super::part_one(&input);
        assert_eq!(result, 357);
        let result = super::part_two(&input);
        assert_eq!(result, 3121910778619);
    }
}
