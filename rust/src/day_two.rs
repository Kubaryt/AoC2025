pub fn run(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();

    let part_one_result = part_one(&lines);
    let part_two_result = part_two(&lines);
    println!("Part One: {}", part_one_result);
    println!("Part Two: {}", part_two_result);
}

fn part_one(lines: &Vec<&str>) -> i64 {
    let mut sum = 0;
    for line in lines {
        let (range_start, range_end) = parse_range(line);
        for number in range_start..=range_end {
            let number_str = number.to_string();
            let length = number_str.len();
            if number_str[0..length / 2] == number_str[length / 2..] {
                sum += number;
            }
        }
    }

    sum
}

fn part_two(lines: &Vec<&str>) -> i64 {
    let mut sum = 0;
    for line in lines {
        let (range_start, range_end) = parse_range(line);
        for number in range_start..=range_end {
            let number_str = number.to_string();
            let length = number_str.len();
            for n in 0..length / 2 {
                let pattern = &number_str[0..=n];
                if number_str == pattern.repeat(length / (n + 1)) {
                    sum += number;
                    break;
                }
            }
        }
    }

    sum
}

fn parse_range(line: &str) -> (i64, i64) {
    let mut line = line.split("-");
    let range_start = line.next().unwrap().parse::<i64>().unwrap();
    let range_end = line.next().unwrap().parse::<i64>().unwrap();
    (range_start, range_end)
}

mod tests {
    #[test]
    fn test_input() {
        let input = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];
        let result = super::part_one(&input);
        assert_eq!(result, 1227775554);
        let result = super::part_two(&input);
        assert_eq!(result, 4174379265)
    }
}
