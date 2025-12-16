use memoize::memoize;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for bank in input.lines() {
        let mut firsti: usize = 0;
        let mut first: char = '0';
        for i in ('0'..='9').rev() {
            if let Some(x) = bank[..bank.len() - 1].find(i) {
                firsti = x;
                first = i;
                break;
            }
        }
        let mut second: char = '0';
        for i in ('0'..='9').rev() {
            if bank[firsti + 1..].find(i).is_some() {
                second = i;
                break;
            }
        }
        let v = (first as u64 - '0' as u64) * 10 + (second as u64 - '0' as u64);
        total += v;
    }

    Some(total)
}

#[memoize]
fn find_largest(input: String, i: usize, d: u64) -> u64 {
    if i == input.len() || d == 0 {
        return 0;
    }
    let v = input.chars().nth(i).unwrap() as u64 - '0' as u64;
    find_largest(input.clone(), i + 1, d).max(v + 10 * find_largest(input, i + 1, d - 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for bank in input.lines() {
        let bank: String = bank.chars().rev().collect();
        total = total.strict_add(find_largest(bank.clone(), 0, 12));
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
