use memoize::memoize;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
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
        let v = (first as usize - '0' as usize) * 10 + (second as usize - '0' as usize);
        total += v;
    }

    Some(total)
}

#[memoize]
fn f(mut input: String, d: usize) -> usize {
    if input.is_empty() || d == 0 {
        return 0;
    }
    let v = input.pop().unwrap() as usize - '0' as usize;
    f(input.clone(), d).max(v + 10 * f(input, d - 1))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().map(|l| f(l.to_string(), 12)).sum())
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
