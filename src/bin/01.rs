advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let mut dial: usize = 50;
    let mut count = 0;
    for line in input.lines() {
        let left = line.get(..1).unwrap() == "L";
        let mut amt: usize = line.get(1..).unwrap().parse().unwrap();

        if left {
            if amt > 100 {
                amt %= 100;
            }
            amt = 100 - amt;
        }

        dial += amt;
        dial %= 100;

        if dial == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut dial: i32 = 50;
    let mut count = 0;
    for line in input.lines() {
        let left = line.get(..1).unwrap() == "L";
        let amt: i32 = line.get(1..).unwrap().parse().unwrap();

        if left {
            dial -= amt;
            while dial < 0 {
                dial += 100;
                count += 1;
            }
        } else {
            dial += amt;
            while dial > 99 {
                dial -= 100;
                count += 1;
            }
        }
    }

    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
