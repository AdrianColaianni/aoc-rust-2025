use std::collections::{BinaryHeap, HashSet};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().filter_map(part_one_line).sum())
}

fn part_one_line(input: &str) -> Option<usize> {
    let input: Vec<&str> = input.split(' ').collect();

    let mut goal = &input[0][1..];
    goal = &goal[..goal.len() - 1];
    let goal: Vec<bool> = goal.chars().map(|c| c == '#').collect();

    let buttons: Vec<Vec<usize>> = input[1..input.len() - 1]
        .iter()
        .map(|s| {
            s[1..s.len() - 1]
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();

    let mut queue = HashSet::new();
    queue.insert(vec![false; goal.len()]);
    let mut c = 0;

    loop {
        if queue.contains(&goal) {
            return Some(c);
        }

        c += 1;

        let mut new_queue = HashSet::new();
        for machine in queue {
            for b in &buttons {
                let mut m = machine.clone();
                for n in b {
                    m[*n] = !m[*n];
                }
                new_queue.insert(m);
            }
        }
        queue = new_queue;
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().filter_map(part_two_line).sum())
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct Machine {
    depth: usize,
    cost: usize,
    state: Vec<usize>,
}

impl Machine {
    fn new(state: Vec<usize>, depth: usize) -> Self {
        let mut cost = state.iter().sum();
        cost /= state.len();
        Self { state, cost, depth }
    }

    fn move_by(&self, button: &Vec<usize>) -> Option<Self> {
        let mut state = self.state.clone();
        for b in button {
            state[*b] = state[*b].checked_sub(1)?;
        }
        Some(Self::new(state, self.depth + 1))
    }
}

impl PartialOrd for Machine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.depth.partial_cmp(&self.depth) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Machine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part_two_line(input: &str) -> Option<usize> {
    let input: Vec<&str> = input.split(' ').collect();

    let mut goal = &input.last().unwrap()[1..];
    goal = &goal[..goal.len() - 1];
    let goal: Machine = Machine::new(goal.split(',').map(|c| c.parse().unwrap()).collect(), 0);

    let mut buttons: Vec<Vec<usize>> = input[1..input.len() - 1]
        .iter()
        .map(|s| {
            s[1..s.len() - 1]
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();

    // Buttons that do the most first
    // buttons.sort_by(|a, b| b.iter().sum::<usize>().cmp(&a.iter().sum()));

    println!("{:?}: {:?}", goal, buttons);

    let mut queue = BinaryHeap::new();
    queue.push(goal);

    while let Some(machine) = queue.pop() {
        // println!("Checking {:?} of {}", machine, queue.len());
        for b in &buttons {
            if let Some(m) = machine.move_by(b) {
                if m.cost == 0 {
                    return Some(m.depth);
                }
                queue.push(m);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}

