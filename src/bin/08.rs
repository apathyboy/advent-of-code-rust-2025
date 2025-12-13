use glam::IVec3;
use std::cmp::Reverse;

advent_of_code::solution!(8);

fn parse_junction_box(line: &str) -> Option<IVec3> {
    let elems = line
        .split(',')
        .take(3)
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Some(IVec3::new(elems[0], elems[1], elems[2]))
}

fn pair_dist2(a: IVec3, b: IVec3) -> i64 {
    let dx = (a.x - b.x) as i64;
    let dy = (a.y - b.y) as i64;
    let dz = (a.z - b.z) as i64;
    dx * dx + dy * dy + dz * dz
}

/// Returns all unordered pairs (i, j) with i < j, sorted by distance ascending.
fn all_pairs_sorted(points: &[IVec3]) -> Vec<((usize, usize), i64)> {
    let n = points.len();
    let mut pairs = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            let d2 = pair_dist2(points[i], points[j]);
            pairs.push(((i, j), d2));
        }
    }

    // Sort by squared distance
    pairs.sort_by_key(|&(_, d2)| d2);
    pairs
}
#[derive(Clone, Debug)]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut coords = value.split(',').map(|coord| coord.parse().unwrap());

        Position {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }
}

impl Position {
    fn distance(&self, other: &Position) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

struct Node {
    parent: usize,
    size: usize,
}

struct Dsu {
    nodes: Vec<Node>,
}

impl Dsu {
    fn new(len: usize) -> Self {
        Self {
            nodes: (0..len).map(|parent| Node { parent, size: 1 }).collect(),
        }
    }

    fn parent(&mut self, mut x: usize) -> usize {
        loop {
            // get parent
            let parent = self.nodes[x].parent;
            // reached top most parent
            if parent == x {
                break parent;
            }
            // update parent
            self.nodes[x].parent = self.nodes[parent].parent;
            // update x
            x = parent;
        }
    }

    fn add_pair(&mut self, u: usize, v: usize) -> usize {
        let (mut pu, mut pv) = (self.parent(u), self.parent(v));

        // same parent => same subset
        if pu == pv {
            return self.nodes[pu].size;
        }

        // make sure pu is bigger than pv
        if self.nodes[pu].size < self.nodes[pv].size {
            std::mem::swap(&mut pu, &mut pv);
        }

        self.nodes[pv].parent = pu;
        self.nodes[pu].size += self.nodes[pv].size;
        self.nodes[pu].size
    }
}

fn get_closest_pairs<const N: usize>(boxes: &[Position]) -> Vec<(usize, usize, u64)> {
    // (start_idx, end_idx, distance)
    let mut pairs: Vec<(usize, usize, u64)> = Vec::with_capacity(N);

    let mut iter = (0..boxes.len()).flat_map(|i| (i + 1..boxes.len()).map(move |j| (i, j)));

    for _ in 0..N {
        let (i, j) = iter.next().unwrap();
        let d = boxes[i].distance(&boxes[j]);
        pairs.push((i, j, d));
    }

    pairs.sort_unstable_by_key(|(_, _, d)| *d);

    for (i, j) in iter {
        let d = boxes[i].distance(&boxes[j]);

        if d > pairs.last().unwrap().2 {
            continue;
        }

        pairs.pop();
        let insert_idx = pairs.partition_point(|(_, _, dist)| dist < &d);
        pairs.insert(insert_idx, (i, j, d));
    }

    pairs
}

pub fn part_one(input: &str) -> Option<usize> {
    let connections = if cfg!(test) {
        // when compiled as part of `cargo test`
        10
    } else {
        // when compiled for `cargo run`, `cargo build`, etc.
        1000
    };

    let junction_boxes: Vec<IVec3> = input.lines().filter_map(parse_junction_box).collect();
    let mut circuits: Vec<Vec<IVec3>> = Vec::new();

    let pairs = all_pairs_sorted(&junction_boxes);

    for ((a, b), _dist) in pairs.iter().take(connections) {
        //println!(
        //    "Comparison {:?}, {:?}",
        //    junction_boxes[*a], junction_boxes[*b]
        //);

        let box_a = junction_boxes[*a];
        let box_b = junction_boxes[*b];

        //println!("Shortest distance {:?}, {:?}", box_a, box_b);

        if let Some(circuit_a_idx) = circuits.iter().position(|circuit| circuit.contains(&box_a)) {
            if let Some(circuit_b_idx) =
                circuits.iter().position(|circuit| circuit.contains(&box_b))
            {
                // both are in a circuit.. make sure they aren't the same circuit:
                if circuit_a_idx != circuit_b_idx {
                    // not in the same circuit
                    let mut circuit_b = circuits.remove(circuit_b_idx);
                    //println!(
                    //    "Combining circuits {:?}, {:?}",
                    //    circuits[circuit_a_idx], circuit_b
                    //);
                    let idx = if circuit_b_idx < circuit_a_idx {
                        circuit_a_idx - 1
                    } else {
                        circuit_a_idx
                    };
                    circuits[idx].append(&mut circuit_b)
                }
                //else {
                //    println!("In same circuit: {:?}, {:?}", box_a, box_b);
                //}
            } else {
                //println!(
                //    "Adding {:?} to circuit {:?}",
                //    box_b, circuits[circuit_a_idx]
                //);

                circuits[circuit_a_idx].push(box_b);
            }
        } else if let Some(circuit_b_idx) =
            circuits.iter().position(|circuit| circuit.contains(&box_b))
        {
            //println!(
            //    "Adding {:?} to circuit {:?}",
            //    box_a, circuits[circuit_b_idx]
            //);

            circuits[circuit_b_idx].push(box_a);
        } else {
            //println!("Creating new circuit [{:?}, {:?}]", box_a, box_b);
            circuits.push(Vec::from([box_a, box_b]));
        }

        //println!("Circuits:");
        //for circuit in &circuits {
        //    println!("{:?}", circuit);
        //}

        //println!();
        //println!();
    }

    circuits.sort_by_cached_key(|i| Reverse(i.len()));

    //for i in 0..circuits.len() {
    //    println!("Circuit {} -> {}", i, circuits[i].len());
    //}

    Some(circuits[0].len() * circuits[1].len() * circuits[2].len())
}

#[cfg(test)]
const DEFAULT_N: usize = 100;
#[cfg(not(test))]
const DEFAULT_N: usize = 5_000;

pub fn part_two(input: &str) -> Option<u64> {
    let connections = if cfg!(test) {
        // when compiled as part of `cargo test`
        20
    } else {
        // when compiled for `cargo run`, `cargo build`, etc.
        1000
    };

    let boxes: Vec<Position> = input.lines().map(Position::from).collect();

    // (start_idx, end_idx, distance)
    let pairs: Vec<(usize, usize, u64)> = get_closest_pairs::<DEFAULT_N>(&boxes);

    let mut circuits = Dsu::new(connections);

    for (i, j, _) in pairs {
        if circuits.add_pair(i, j) == connections {
            return Some(boxes[i].x * boxes[j].x);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
