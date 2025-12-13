use std::collections::{HashMap, VecDeque};

use pathfinding::prelude::topological_sort;

advent_of_code::solution!(11);

fn all_paths_dag<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    goal: &'a str,
) -> Vec<Vec<&'a str>> {
    // 1) Topologically order the reachable subgraph (will error if a cycle exists)
    let order = topological_sort(&[start], |n| graph.get(n).cloned().unwrap_or_default())
        .expect("graph must be acyclic"); // DAG assumption

    // 2) DP cache: paths_from[node] = all paths starting at node and ending at goal
    let mut paths_from: HashMap<&'a str, Vec<Vec<&'a str>>> = HashMap::new();

    for &node in order.iter().rev() {
        if node == goal {
            paths_from.insert(node, vec![vec![goal]]);
            continue;
        }

        let mut acc: Vec<Vec<&'a str>> = Vec::new();
        if let Some(succs) = graph.get(node) {
            for &s in succs {
                if let Some(suffixes) = paths_from.get(s) {
                    for suf in suffixes {
                        let mut p = Vec::with_capacity(suf.len() + 1);
                        p.push(node);
                        p.extend_from_slice(suf);
                        acc.push(p);
                    }
                }
            }
        }
        paths_from.insert(node, acc);
    }

    paths_from.remove(start).unwrap_or_default()
}

const REQ: u8 = 0b11;

fn bit(node: &str, a: &str, b: &str) -> u8 {
    (if node == a { 0b01 } else { 0 }) | (if node == b { 0b10 } else { 0 })
}

fn intern_graph<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
) -> (Vec<&'a str>, Vec<Vec<usize>>, HashMap<&'a str, usize>) {
    let mut id_of: HashMap<&'a str, usize> = HashMap::new();
    let mut names: Vec<&'a str> = Vec::new();

    let get_id = |s: &'a str, id_of: &mut HashMap<&'a str, usize>, names: &mut Vec<&'a str>| {
        if let Some(&id) = id_of.get(s) {
            id
        } else {
            let id = names.len();
            names.push(s);
            id_of.insert(s, id);
            id
        }
    };

    // Assign ids for all keys and neighbors
    for (&u, neigh) in graph {
        get_id(u, &mut id_of, &mut names);
        for &v in neigh {
            get_id(v, &mut id_of, &mut names);
        }
    }

    let mut adj = vec![Vec::<usize>::new(); names.len()];
    for (&u, neigh) in graph {
        let uid = id_of[&u];
        let row = &mut adj[uid];
        row.reserve(neigh.len());
        for &v in neigh {
            row.push(id_of[&v]);
        }
    }

    (names, adj, id_of)
}

/// Topological order of nodes reachable from start (Kahn’s algorithm).
fn topo_reachable(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = adj.len();

    // Mark reachable
    let mut reachable = vec![false; n];
    let mut stack = vec![start];
    reachable[start] = true;
    while let Some(u) = stack.pop() {
        for &v in &adj[u] {
            if !reachable[v] {
                reachable[v] = true;
                stack.push(v);
            }
        }
    }

    // Indegrees within reachable subgraph
    let mut indeg = vec![0usize; n];
    for u in 0..n {
        if !reachable[u] {
            continue;
        }
        for &v in &adj[u] {
            if reachable[v] {
                indeg[v] += 1;
            }
        }
    }

    let mut q = VecDeque::new();
    for u in 0..n {
        if reachable[u] && indeg[u] == 0 {
            q.push_back(u);
        }
    }

    let mut order = Vec::new();
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            if !reachable[v] {
                continue;
            }
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }

    order
}

/// dp[u][mask] = number of valid paths from u to goal, given `mask` already collected BEFORE u.
fn count_paths_must_visit(
    graph: &HashMap<&str, Vec<&str>>,
    start: &str,
    goal: &str,
    must_a: &str,
    must_b: &str,
) -> u64 {
    let (names, adj, id_of) = intern_graph(graph);

    let Some(&s) = id_of.get(start) else {
        return 0;
    };
    let Some(&g) = id_of.get(goal) else {
        return 0;
    };

    let order = topo_reachable(&adj, s);

    // If goal isn't reachable, count is 0
    if !order.contains(&g) {
        return 0;
    }

    let mut dp = vec![[0u64; 4]; names.len()];

    for &u in order.iter().rev() {
        let ub = bit(names[u], must_a, must_b);

        for mask in 0..4u8 {
            let newmask = mask | ub;

            if u == g {
                dp[u][mask as usize] = if newmask == REQ { 1 } else { 0 };
                continue;
            }

            let mut sum = 0u64;
            for &v in &adj[u] {
                // “mask before v” is newmask (because u is on the path before v)
                sum = sum.saturating_add(dp[v][newmask as usize]);
            }
            dp[u][mask as usize] = sum;
        }
    }

    dp[s][0]
}

pub fn part_one(input: &str) -> Option<u64> {
    let server_rack: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|s| {
            let (device, connections) = s
                .split_once(": ")
                .expect("expected device connection mapping");

            (device, connections.split(' ').collect())
        })
        .collect();

    let paths = all_paths_dag(&server_rack, "you", "out");

    Some(paths.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let server_rack: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|s| {
            let (device, connections) = s
                .split_once(": ")
                .expect("expected device connection mapping");

            (device, connections.split(' ').collect())
        })
        .collect();

    let paths = count_paths_must_visit(&server_rack, "svr", "out", "dac", "fft");

    Some(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
