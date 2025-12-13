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

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
