use glam::IVec2;
use std::cmp::Reverse;

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

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn get_max_area(points: &[Point]) -> i32 {
    let length = points.len();
    let mid_top = length / 2;
    let mid_bot = mid_top + 1;

    // ################## TOP HALF ##################
    // The corner of the rectangle in the top half
    let mut corner = points[mid_top];

    // Find the first point that is to the left of the corner with binary search
    let mut lo = 0;
    let mut hi = mid_top / 2;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if points[mid].x >= corner.x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let mut y_bound = points[lo].y;

    // Find the other corner of the rectangle
    let mut j = mid_top - 1;
    let mut max_x = 0;
    let mut max_area = 0;
    while points[j].y <= y_bound {
        // If we have a new highest x coordinate, it is possible this rectangle is the highest area, so we compute it now
        if points[j].x >= max_x {
            max_x = points[j].x;
            max_area = i32::max(
                max_area,
                (corner.x - max_x + 1) * (points[j].y - corner.y + 1),
            );
        }
        j -= 1;
    }

    // ################# BOTTOM HALF ##################
    // The corner of the rectangle in the top half
    corner = points[mid_bot];

    // Find the first point that is to the left of the corner with binary search
    lo = (length + mid_bot) / 2;
    hi = length - 1;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if points[mid].x >= corner.x {
            hi = mid - 1;
        } else {
            lo = mid;
        }
    }
    y_bound = points[lo].y;

    // Find the other corner of the rectangle
    j = mid_bot + 1;
    max_x = 0;
    while points[j].y >= y_bound {
        // If we have a new highest x coordinate, it is possible this rectangle is the highest area, so we compute it now
        if points[j].x >= max_x {
            max_x = points[j].x;
            max_area = i32::max(
                max_area,
                (corner.x - max_x + 1) * (corner.y - points[j].y + 1),
            );
        }
        j += 1
    }
    max_area
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(',').collect();
            Point::new(str::parse(split[0]).unwrap(), str::parse(split[1]).unwrap())
        })
        .collect();
    let max_area = get_max_area(&points);

    Some(max_area as u64)
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
        assert_eq!(result, None);
    }
}
