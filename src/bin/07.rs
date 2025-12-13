use std::{cmp::max, collections::HashSet};

use glam::IVec2;

advent_of_code::solution!(7);

fn fire_beam(
    start: IVec2,
    max_height: usize,
    splitters: &Vec<IVec2>,
    beams: &mut Vec<(IVec2, IVec2)>,
    splits: &mut HashSet<IVec2>,
) {
    let mut split = false;
    for y in (start.y + 1)..(max_height + 1) as i32 {
        if splitters.contains(&IVec2::new(start.x, y)) {
            splits.insert(IVec2::new(start.x, y));

            beams.push((start, IVec2::new(start.x, y - 1)));
            split = true;

            if beams
                .iter()
                .filter(|(beam_start, beam_end)| {
                    beam_start.x == start.x - 1 && y >= beam_start.y && y <= beam_end.y
                })
                .count()
                == 0
            {
                fire_beam(
                    IVec2::new(start.x - 1, y),
                    max_height,
                    splitters,
                    beams,
                    splits,
                );
            }

            if beams
                .iter()
                .filter(|(beam_start, beam_end)| {
                    beam_start.x == start.x + 1 && y >= beam_start.y && y <= beam_end.y
                })
                .count()
                == 0
            {
                fire_beam(
                    IVec2::new(start.x + 1, y),
                    max_height,
                    splitters,
                    beams,
                    splits,
                );
            }
        }

        if split {
            break;
        }
    }

    if !split {
        beams.push((start, IVec2::new(start.x, (max_height - 1) as i32)));
    }
}

fn _fire_quantum_beam(beam: IVec2, max_height: i32, splitters: &HashSet<IVec2>, beams: &mut usize) {
    // We assume beam is non-empty; otherwise this was already UB with unwrap()
    let mut pos = beam.clone();

    for y in pos.y + 1..max_height {
        pos.y = y;

        // Check if current position is a splitter
        if splitters.contains(&pos) {
            // ---- Right branch ----
            // Temporarily extend the beam to the right and recurse
            _fire_quantum_beam(IVec2::new(pos.x + 1, pos.y), max_height, splitters, beams);

            // ---- Main branch deflects left ----
            pos.x -= 1;
        }
    }

    // Store this complete beam path
    *beams += 1;

    //println!("Beams {}", beams);
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut start = IVec2::ZERO;
    let mut splitters: Vec<IVec2> = Vec::new();
    let mut beams: Vec<(IVec2, IVec2)> = Vec::new();
    let mut max_y = 0;
    let mut splits = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => start = IVec2::new(x as i32, y as i32),
                '^' => splitters.push(IVec2::new(x as i32, y as i32)),
                _ => {}
            }
        }

        max_y = y;
    }

    fire_beam(start, max_y + 1, &splitters, &mut beams, &mut splits);

    Some(splits.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid_height = input.lines().count();
    let grid_width = input.lines().next().unwrap().len();
    let mut dp: Vec<Vec<u64>> = Vec::new();
    let mut default_col = Vec::with_capacity(grid_width);
    default_col.resize(grid_width, 0);
    dp.resize(grid_height, default_col);

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                dp[y][x] = 1;
                continue;
            }

            if ch == '.' && y as isize - 1 >= 0 {
                dp[y][x] += dp[y - 1][x];
                continue;
            }

            if ch == '^' {
                for j in [-1_isize, 1_isize] {
                    let new_col = (x as isize + j) as usize;
                    dp[y][new_col] = max(dp[y - 1][x] + dp[y][new_col], dp[y][new_col]);
                }
            }
        }
    }

    Some(dp[grid_height - 1].iter().sum::<u64>())
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
