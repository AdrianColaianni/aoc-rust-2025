advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (range, food) = input.split_once("\n\n").unwrap();
    let mut ranges = Vec::new();
    for r in range.lines() {
        let (s, e) = r.split_once('-').unwrap();
        let s: usize = s.parse().unwrap();
        let e: usize = e.parse().unwrap();
        ranges.push(s..=e);
    }

    let mut v = 0;
    'f: for f in food.lines() {
        let f: usize = f.parse().unwrap();
        for r in &ranges {
            if r.contains(&f) {
                v += 1;
                continue 'f;
            }
        }
    }

    Some(v)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
