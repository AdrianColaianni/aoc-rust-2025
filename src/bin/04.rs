advent_of_code::solution!(4);

fn check_sides(input: &Vec<Vec<bool>>, p: (usize, usize), sides: &Vec<(isize, isize)>) -> bool {
    if !input[p.0][p.1] {
        return false;
    }
    let mut v = 0;
    for s in sides {
        let r = (p.0 as isize + s.0) as usize;
        let c = (p.1 as isize + s.1) as usize;
        if input[r][c] {
            v += 1;
        }
    }
    v < 4
}

pub fn part_one(input: &str) -> Option<u64> {
    let sides: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];
    let left_sides: Vec<(isize, isize)> = sides
        .iter()
        .filter(|i| i.1 != -1)
        .map(|i| i.to_owned())
        .collect();
    let right_sides: Vec<(isize, isize)> = sides
        .iter()
        .filter(|i| i.1 != 1)
        .map(|i| i.to_owned())
        .collect();
    let top_sides: Vec<(isize, isize)> = sides
        .iter()
        .filter(|i| i.0 != -1)
        .map(|i| i.to_owned())
        .collect();
    let bottom_sides: Vec<(isize, isize)> = sides
        .iter()
        .filter(|i| i.0 != 1)
        .map(|i| i.to_owned())
        .collect();

    let input: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();

    let mut sum = 0;
    let rm = input.len() - 1;
    let cm = input[0].len() - 1;

    // Middle
    for r in 1..rm {
        for c in 1..cm {
            if check_sides(&input, (r, c), &sides) {
                sum += 1;
            }
        }
    }

    // Corners are always accessible
    for (r, c) in [(0, 0), (0, cm), (rm, cm), (rm, 0)] {
        if input[r][c] {
            sum += 1;
        }
    }

    // Edges
    for r in 1..rm {
        if check_sides(&input, (r, 0), &left_sides) {
            sum += 1;
        }
        if check_sides(&input, (r, cm), &right_sides) {
            sum += 1;
        }
    }
    for c in 1..cm {
        if check_sides(&input, (0, c), &top_sides) {
            sum += 1;
        }
        if check_sides(&input, (rm, c), &bottom_sides) {
            sum += 1;
        }
    }

    Some(sum)
}

fn find(input: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let sides: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];
    let left_sides: Vec<(isize, isize)> = sides
        .iter()
        .filter(|i| i.1 != -1)
        .map(|i| i.to_owned())
        .collect();
    let right_sides: Vec<(isize, isize)> = sides
        .iter()
        .filter(|i| i.1 != 1)
        .map(|i| i.to_owned())
        .collect();
    let top_sides: Vec<(isize, isize)> = sides
        .iter()
        .filter(|i| i.0 != -1)
        .map(|i| i.to_owned())
        .collect();
    let bottom_sides: Vec<(isize, isize)> = sides
        .iter()
        .filter(|i| i.0 != 1)
        .map(|i| i.to_owned())
        .collect();

    let rm = input.len() - 1;
    let cm = input[0].len() - 1;

    // Middle
    for r in 1..rm {
        for c in 1..cm {
            if check_sides(&input, (r, c), &sides) {
                res.push((r, c));
            }
        }
    }

    // Corners are always accessible
    for (r, c) in [(0, 0), (0, cm), (rm, cm), (rm, 0)] {
        if input[r][c] {
            res.push((r, c));
        }
    }

    // Edges
    for r in 1..rm {
        if check_sides(&input, (r, 0), &left_sides) {
            res.push((r, 0));
        }
        if check_sides(&input, (r, cm), &right_sides) {
            res.push((r, cm));
        }
    }
    for c in 1..cm {
        if check_sides(&input, (0, c), &top_sides) {
            res.push((0, c));
        }
        if check_sides(&input, (rm, c), &bottom_sides) {
            res.push((rm, c));
        }
    }

    res
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();

    let mut t = 0;
    loop {
        let r = find(&input);
        if r.is_empty() {
            break;
        }
        t += r.len();
        for r in r {
            input[r.0][r.1] = false;
        }
    }

    Some(t as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
