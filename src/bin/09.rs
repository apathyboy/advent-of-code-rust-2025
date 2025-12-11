use std::cmp::Reverse;

use glam::IVec2;

advent_of_code::solution!(9);

fn parse_tile_position(line: &str) -> Option<IVec2> {
    let elems = line
        .split(',')
        .take(2)
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Some(IVec2::new(elems[0], elems[1]))
}

fn all_pairs_sorted(points: &[IVec2]) -> Vec<((usize, usize), u64)> {
    let n = points.len();
    let mut pairs = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            let width = ((points[i].x - points[j].x) + 1).abs() as u64;
            let height = ((points[i].y - points[j].y) + 1).abs() as u64;

            let area = width * height;

            pairs.push(((i, j), area));
        }
    }

    // Sort by squared distance
    pairs.sort_by_key(|&(_, area)| Reverse(area));
    pairs
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles: Vec<IVec2> = input.lines().filter_map(parse_tile_position).collect();

    let areas = all_pairs_sorted(&tiles);
    let ((_, _), area) = areas.first()?;

    Some(*area)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
