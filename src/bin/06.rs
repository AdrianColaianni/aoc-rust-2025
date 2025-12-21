advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = input.lines().rev().into_iter();
    let ops: Vec<bool> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s == "*")
        .collect();
    let mut totals = vec![0; ops.len()];

    // Prepare for multiplication
    for i in 0..totals.len() {
        if ops[i] {
            totals[i] = 1;
        }
    }

    for l in input {
        for (i, n) in l.split_whitespace().enumerate() {
            let n: usize = n.parse().unwrap();
            if ops[i] {
                totals[i] *= n;
            } else {
                totals[i] += n;
            }
        }
    }

    Some(totals.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input: Vec<&str> = input.lines().collect();
    let ops: Vec<bool> = input
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|s| s == "*")
        .collect();
    let mut totals = vec![0; ops.len()];
    let mut i = 0;

    // Prepare for multiplication
    for i in 0..totals.len() {
        if ops[i] {
            totals[i] = 1;
        }
    }

    let input: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
    let cap = input.len();

    let mut s = String::with_capacity(cap);
    for c in 0..input[0].len() {
        for r in 0..cap {
            s.push(input[r][c]);
        }
        let st = s.trim();
        if st.is_empty() {
            i += 1;
        } else {
            let s: usize = st.parse().unwrap();
            if ops[i] {
                totals[i] *= s;
            } else {
                totals[i] += s;
            }
        }
        s.clear();

    }

    Some(totals.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
