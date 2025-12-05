use std::cmp::max;

pub fn run(input: &str) {
    let lines: Vec<&str> = input.split("\n\n").collect();
    let ranges: Vec<&str> = lines[0].lines().collect();
    let values: Vec<usize> = lines[1].lines().map(|n| n.parse().unwrap()).collect();

    let part_one_result = part_one(&ranges, &values);
    println!("Part One Result: {}", part_one_result);
    let part_two_result = part_two(&ranges);
    println!("Part Two Result: {}", part_two_result);
}

fn part_one(ranges: &Vec<&str>, values: &Vec<usize>) -> usize {
    let mut fresh_count = 0;
    let mut is_fresh;
    for value in values {
        is_fresh = false;
        for range in ranges {
            let (start, end) = parse_range(range);
            if &{ start } <= value && value <= &{ end } {
                is_fresh = true;
                break;
            }
        }
        if is_fresh {
            fresh_count += 1
        }
    }

    fresh_count
}

fn part_two(ranges: &Vec<&str>) -> usize {
    let mut total_fresh = 0;
    let mut ranges = ranges
        .iter()
        .map(|range| parse_range(range))
        .collect::<Vec<(usize, usize)>>();
    ranges.sort();
    let mut current_max_right = 0;
    for (start, end) in ranges {
        let end = end + 1;
        total_fresh += max(0, (end as isize) - (max(start, current_max_right) as isize)) as usize;
        current_max_right = max(current_max_right, end);
    }

    total_fresh
}

fn parse_range(line: &str) -> (usize, usize) {
    let mut line = line.split("-");
    let range_start = line.next().unwrap().parse::<usize>().unwrap();
    let range_end = line.next().unwrap().parse::<usize>().unwrap();
    (range_start, range_end)
}

mod test {
    #[test]
    fn test_with_test_input() {
        let ranges = vec!["3-5", "10-14", "16-20", "12-18"];
        let values = vec![1usize, 5, 8, 11, 17, 32];
        let result = super::part_one(&ranges, &values);
        assert_eq!(result, 3);
        let result = super::part_two(&ranges);
        assert_eq!(result, 14);
    }
}
