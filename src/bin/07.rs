advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = input.lines().into_iter();
    let start = input.next().unwrap();
    let len = start.len();
    let start = start
        .char_indices()
        .skip_while(|(_, c)| *c != 'S')
        .next()
        .unwrap()
        .0;
    let mut beams = vec![false; len];
    beams[start] = true;
    let mut was_split = 0;

    for l in input {
        for (i, c) in l.char_indices() {
            if !beams[i] {
                continue;
            }
            if c == '^' {
                was_split += 1;
                if let Some(i) = i.checked_sub(1) {
                    beams[i] = true;
                }
                if i + 1 != len {
                    beams[i + 1] = true;
                }
                beams[i] = false;
            }
        }
    }

    Some(was_split)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = input.lines().into_iter();
    let start = input.next().unwrap();
    let len = start.len();
    let start = start
        .char_indices()
        .skip_while(|(_, c)| *c != 'S')
        .next()
        .unwrap()
        .0;
    let mut beams = vec![0; len];
    beams[start] = 1;

    for l in input {
        for (i, c) in l.char_indices() {
            if beams[i] == 0 {
                continue;
            }
            if c == '^' {
                if let Some(o) = i.checked_sub(1) {
                    beams[o] += beams[i];
                }
                if i + 1 != len {
                    beams[i + 1] += beams[i];
                }
                beams[i] = 0;
            }
        }
    }

    Some(beams.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
