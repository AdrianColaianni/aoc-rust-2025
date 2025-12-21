use petgraph::{Graph, visit::Dfs};
use std::sync::OnceLock;

static CONNECTION_COUNT: OnceLock<usize> = OnceLock::new();

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for Pos {
    fn from(value: &str) -> Self {
        let v: Vec<usize> = value.splitn(3, ',').map(|s| s.parse().unwrap()).collect();
        Pos {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:3},{:3},{:3}", self.x, self.y, self.z))
    }
}

impl Pos {
    fn dist(&self, v: &Self) -> f32 {
        // There's probably a better way to do this
        let a = self.x.abs_diff(v.x);
        let b = self.y.abs_diff(v.y);
        let c = ((a.pow(2) + b.pow(2)) as f32).sqrt();
        let d = self.z.abs_diff(v.z);
        (c.powi(2) + d.pow(2) as f32).sqrt()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut g: Graph<Pos, f32, _> = Graph::new_undirected();

    // Build graph
    for l in input.lines() {
        g.add_node(l.into());
    }
    let is: Vec<_> = g.node_indices().collect();
    for i in 0..is.len() {
        for j in i + 1..is.len() {
            g.add_edge(is[i], is[j], g[is[i]].dist(&g[is[j]]));
        }
    }

    // Trim edges
    let mut v: Vec<_> = g.edge_indices().map(|i| (i, g[i])).collect();
    v.sort_by(|x, y| x.1.total_cmp(&y.1));
    v.truncate(*CONNECTION_COUNT.get_or_init(|| 1_000));

    let v: Vec<_> = v.iter().map(|i| i.0).collect();
    g.retain_edges(|_, e| v.contains(&e));

    // Find 3 biggest circuits
    let mut vn: Vec<_> = Vec::new(); // Visited nodes
    let mut c: Vec<usize> = Vec::new(); // Cirtuit size
    for n in g.node_indices() {
        if g.neighbors(n).count() == 0 || vn.contains(&n) {
            continue;
        }
        let mut ct = 0;
        let mut dfs = Dfs::new(&g, n);
        while let Some(nx) = dfs.next(&g) {
            vn.push(nx);
            ct += 1;
        }
        c.push(ct);
    }

    c.sort_by(|x, y| y.cmp(&x));
    c.truncate(3);
    let mut ans = 1;
    for a in c {
        ans *= a;
    }
    Some(ans)
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
        CONNECTION_COUNT.get_or_init(|| 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        CONNECTION_COUNT.get_or_init(|| 10);
        assert_eq!(result, None);
    }
}
