advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    for range in input.split(',') {
        let mut range = range.split('-');
        let start: u64 = range.next().unwrap().trim().parse().unwrap();
        let end: u64 = range.next().unwrap().trim().parse().unwrap();
        for i in start..=end {
            let l = 10_u64.pow(len(i) / 2);
            if i / l == i % l {
                count += i;
            }
        }
    }
    Some(count)
}

fn len(n: u64) -> u32 {
    let mut p = 10;
    let mut c = 1;
    while n >= p {
        p = p * 10;
        c += 1;
    }
    c
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0;
    for range in input.split(',') {
        let mut range = range.split('-');
        let start: u64 = range.next().unwrap().trim().parse().unwrap();
        let end: u64 = range.next().unwrap().trim().parse().unwrap();
        for num in start..=end {
            let l = len(num);
            // println!("{}: {}", num, l);
            'len: for len in (1..=l / 2).filter(|d| l % d == 0) {
                // println!("- {}", len);
                let f: Vec<_> = (0..l).step_by(len as usize).collect();
                for p in f.windows(2) {
                    // println!("  - {:?},{:?}", p[0]..p[0] + len, p[1]..p[1] + len)
                    let o = int_at(num, p[0]..p[0] + len);
                    let t = int_at(num, p[1]..p[1] + len);
                    if o != t {
                        continue 'len;
                    }
                }
                // Success!
                count += num;
                break;
            }
        }
    }
    Some(count)
}

fn int_at(n: u64, r: std::ops::Range<u32>) -> u64 {
    let l = len(n) - 1;
    let mut a = 0;

    for (p, i) in r.enumerate() {
        a *= 10;
        let mut t = n / 10_u64.pow(l - i);
        t %= 10;
        a += t;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
