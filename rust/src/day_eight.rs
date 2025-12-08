pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let part_one_result = part_one(&lines, 1000);
    println!("Part One Result: {}", part_one_result);
    let part_two_result = part_two(&lines);
    println!("Part Two Result: {}", part_two_result);
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    a: usize,
    b: usize,
    dist: usize,
}

fn part_one(lines: &Vec<&str>, number_of_connections: usize) -> usize {
    let junction_boxes = lines
        .iter()
        .map(|line| {
            let coords: Vec<usize> = line
                .split(',')
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect();
            (coords[0], coords[1], coords[2])
        })
        .collect::<Vec<(usize, usize, usize)>>();

    let boxes_len = junction_boxes.len();
    let mut parents: Vec<usize> = (0..boxes_len).collect();

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..boxes_len {
        for j in (i + 1)..boxes_len {
            let dx = junction_boxes[i].0 as isize - junction_boxes[j].0 as isize;
            let dy = junction_boxes[i].1 as isize - junction_boxes[j].1 as isize;
            let dz = junction_boxes[i].2 as isize - junction_boxes[j].2 as isize;
            let dist = (dx * dx + dy * dy + dz * dz) as usize;
            edges.push(Edge { a: i, b: j, dist });
        }
    }

    edges.sort_by_key(|edge| edge.dist);

    let mut sizes: Vec<usize> = vec![1; boxes_len];
    for (i, edge) in edges.iter().enumerate() {
        if i == number_of_connections {
            break;
        }
        union(&mut parents, &mut sizes, edge.a, edge.b);
    }

    let mut counts: Vec<usize> = sizes
        .iter()
        .enumerate()
        .filter(|(i, _)| *i == find(&mut parents, *i))
        .map(|(_, &s)| s)
        .collect();

    counts.sort_unstable();
    counts.iter().rev().take(3).copied().product()
}

fn part_two(lines: &Vec<&str>) -> usize {
    let junction_boxes: Vec<(usize, usize, usize)> = lines
        .iter()
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|n| n.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .collect();

    let n = junction_boxes.len();
    let mut parents: Vec<usize> = (0..n).collect();
    let mut sizes: Vec<usize> = vec![1; n];

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = junction_boxes[i].0 as isize - junction_boxes[j].0 as isize;
            let dy = junction_boxes[i].1 as isize - junction_boxes[j].1 as isize;
            let dz = junction_boxes[i].2 as isize - junction_boxes[j].2 as isize;
            let dist = (dx * dx + dy * dy + dz * dz) as usize;
            edges.push(Edge { a: i, b: j, dist });
        }
    }
    edges.sort_by_key(|e| e.dist);

    for edge in edges {
        let merged_size = union(&mut parents, &mut sizes, edge.a, edge.b);
        if merged_size == n {
            return junction_boxes[edge.a].0 * junction_boxes[edge.b].0;
        }
    }

    unreachable!()
}

fn union(parents: &mut Vec<usize>, sizes: &mut [usize], a: usize, b: usize) -> usize {
    let mut a_root = find(parents, a);
    let mut b_root = find(parents, b);
    if a_root == b_root {
        return sizes[a_root];
    }
    if sizes[a_root] < sizes[b_root] {
        std::mem::swap(&mut a_root, &mut b_root);
    }
    parents[b_root] = a_root;
    sizes[a_root] += sizes[b_root];
    sizes[a_root]
}

fn find(parents: &mut Vec<usize>, x: usize) -> usize {
    if parents[x] != x {
        parents[x] = find(parents, parents[x]);
    }
    parents[x]
}

mod test {
    #[test]
    fn test_with_test_input() {
        let input = vec![
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ];
        let result = super::part_one(&input, 10);
        assert_eq!(result, 40);
        let result = super::part_two(&input);
        assert_eq!(result, 25272);
    }
}
