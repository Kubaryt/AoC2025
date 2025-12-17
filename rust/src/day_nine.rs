use std::collections::HashMap;

pub fn run(lines: &str) {
    let lines: Vec<&str> = lines.lines().collect();
    let part_one_result = part_one(&lines);
    println!("Part One: {}", part_one_result);
    let part_two_result = part_two(&lines);
    println!("Part Two: {}", part_two_result);
}

fn part_one(lines: &Vec<&str>) -> usize {
    let coordinates = lines
        .iter()
        .map(|line| {
            let coords: Vec<usize> = line
                .split(',')
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect();
            (coords[0], coords[1])
        })
        .collect::<Vec<(usize, usize)>>();
    let mut rectangles = Vec::new();
    for coords1 in &coordinates {
        for coords2 in &coordinates {
            if coords1 == coords2 {
                continue;
            }
            let x1 = coords1.0.min(coords2.0);
            let y1 = coords1.1.min(coords2.1);
            let x2 = coords1.0.max(coords2.0);
            let y2 = coords1.1.max(coords2.1);
            rectangles.push((x2 - x1 + 1) * (y2 - y1 + 1));
        }
    }
    rectangles.sort();

    *rectangles.last().unwrap()
}

fn part_two(lines: &Vec<&str>) -> usize {
    let coordinates = lines
        .iter()
        .map(|line| {
            let coords: Vec<usize> = line
                .split(',')
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect();
            (coords[0], coords[1])
        })
        .collect::<Vec<(usize, usize)>>();

    let mut is_in_poly_cache: HashMap<(isize, isize), bool> = HashMap::new();
    let mut biggest = 0;
    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let coords1 = coordinates[i];
            let coords2 = coordinates[j];
            if coords1 == coords2 {
                continue;
            }
            let x1 = coords1.0.min(coords2.0);
            let y1 = coords1.1.min(coords2.1);
            let x2 = coords1.0.max(coords2.0);
            let y2 = coords1.1.max(coords2.1);
            let area = (x2 - x1 + 1) * (y2 - y1 + 1);
            if area <= biggest {
                continue;
            }
            let mut valid = true;
            for (x, y) in [(x1, y1), (x1, y2), (x2, y1), (x2, y2)] {
                if !is_in_poly(x as isize, y as isize, &coordinates, &mut is_in_poly_cache) {
                    valid = false;
                    break;
                }
            }
            for (ex1, ey1, ex2, ey2) in coordinates
                .iter()
                .zip(coordinates.iter().cycle().skip(1))
                .map(|(&p1, &p2)| (p1.0, p1.1, p2.0, p2.1))
            {
                if edge_intersects(ex1, ey1, ex2, ey2, x1, y1, x2, y2) {
                    valid = false;
                    break;
                }
            }
            if !valid {
                continue;
            }
            biggest = area;
        }
    }
    biggest
}

fn is_in_poly(
    x: isize,
    y: isize,
    poly: &[(usize, usize)],
    cache: &mut HashMap<(isize, isize), bool>,
) -> bool {
    if let Some(cached) = cache.get(&(x, y)) {
        return *cached;
    }
    let mut inside = false;

    for (x1, y1, x2, y2) in (0..poly.len()).map(|idx| {
        let p1 = poly[idx];
        let p2 = poly[(idx + 1) % poly.len()];
        (p1.0 as isize, p1.1 as isize, p2.0 as isize, p2.1 as isize)
    }) {
        if (x == x1 && x == x2 && y >= y1.min(y2) && y <= y1.max(y2))
            || y == y1 && y == y2 && x >= x1.min(x2) && x <= x1.max(x2)
        {
            cache.insert((x, y), true);
            return true;
        }

        let intersect = ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1);
        if intersect {
            inside = !inside;
        }
    }
    cache.insert((x, y), inside);
    inside
}

#[allow(clippy::too_many_arguments)]
fn edge_intersects(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    rx1: usize,
    ry1: usize,
    rx2: usize,
    ry2: usize,
) -> bool {
    if y1 == y2 {
        if ry1 < y1 && y1 < ry2 && x1.max(x2) > rx1 && x1.min(x2) < rx2 {
            return true;
        }
    } else if x1 == x2 && rx1 < x1 && x1 < rx2 && y1.max(y2) > ry1 && y1.min(y2) < ry2 {
        return true;
    }

    false
}

mod test {
    #[test]
    fn test_with_test_input() {
        let input = vec!["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

        let result = super::part_one(&input);
        assert_eq!(result, 50);
        let result_part_two = super::part_two(&input);
        assert_eq!(result_part_two, 24);
    }

    #[test]
    fn is_in_poly_test() {
        let poly = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        let mut cache = std::collections::HashMap::new();
        assert!(super::is_in_poly(11, 7, &poly, &mut cache));
        assert!(!super::is_in_poly(0, 0, &poly, &mut cache));
        assert!(super::is_in_poly(10, 6, &poly, &mut cache));
        assert!(!super::is_in_poly(10, 8, &poly, &mut cache));
    }
}
