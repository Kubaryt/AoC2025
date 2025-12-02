fn make_rotation(min: i32, max: i32, mut current: i32, rotation: i32) -> (i32, i32) {
    let started_at_zero = current == 0;
    let number_of_wraps = rotation / (max + 1);
    let mut zero_count = number_of_wraps.abs();
    let rotation = rotation - (max + 1) * number_of_wraps;
    current += rotation;
    if current > max {
        current -= max + 1;
        if !started_at_zero {
            zero_count += 1;
        }
    } else if current < min {
        current += max + 1;
        if !started_at_zero {
            zero_count += 1;
        }
    } else if current == 0 {
        zero_count += 1;
    }
    (current, zero_count)
}

fn parse_rotation(line: &str) -> i32 {
    if let Some(stripped) = line.strip_prefix('R') {
        return stripped.parse::<i32>().unwrap();
    };
    if let Some(stripped) = line.strip_prefix('L') {
        -stripped.parse::<i32>().unwrap()
    } else {
        panic!("Invalid rotation: {}", line);
    }
}

fn part_one(lines: &Vec<String>, mut zero_count: i32, mut position: i32) -> i32 {
    for line in lines {
        let rotation = parse_rotation(line);

        (position, _) = make_rotation(0, 99, position, rotation);
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn part_two(lines: &Vec<String>, mut zero_count: i32, mut position: i32) -> i32 {
    for line in lines {
        let rotation = parse_rotation(line);
        let zero_wraps: i32;

        (position, zero_wraps) = make_rotation(0, 99, position, rotation);
        zero_count += zero_wraps;
    }

    zero_count
}

pub fn run(input: &str) {
    let lines = input.lines().map(String::from).collect::<Vec<String>>();
    let zero_count = 0;
    let position = 50;
    println!("Part one: {}", part_one(&lines, zero_count, position));
    println!("Part two: {}", part_two(&lines, zero_count, position));
}

mod tests {
    #[test]
    fn test_make_rotation() {
        use super::make_rotation;

        assert_eq!(make_rotation(0, 99, 50, 228), (78, 2));
        assert_eq!(make_rotation(0, 99, 50, -230), (20, 2));
        assert_eq!(make_rotation(0, 99, 50, 251), (1, 3));
        assert_eq!(make_rotation(0, 99, 50, -251), (99, 3));
        assert_eq!(make_rotation(0, 99, 0, -19), (81, 0));
        assert_eq!(make_rotation(0, 99, 0, -119), (81, 1));
    }

    #[test]
    fn test_run_with_test_input() {
        let input = vec![
            String::from("L68"),
            String::from("L30"),
            String::from("R48"),
            String::from("L5"),
            String::from("R60"),
            String::from("L55"),
            String::from("L1"),
            String::from("L99"),
            String::from("R14"),
            String::from("L82"),
        ];
        let zero_count = 0;
        let position = 50;
        assert_eq!(super::part_one(&input, zero_count, position), 3);
        assert_eq!(super::part_two(&input, zero_count, position), 6);
    }
}
