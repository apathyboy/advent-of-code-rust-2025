use std::collections::HashSet;

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

fn fire_quantum_beam_iterative(start: IVec2, max_height: i32, splitters: &HashSet<IVec2>) -> usize {
    let mut beam_count = 0usize;
    let mut stack: Vec<IVec2> = Vec::new();

    // initial beam
    stack.push(start);

    while let Some(mut pos) = stack.pop() {
        // Simulate this beam’s path
        for y in pos.y + 1..max_height {
            pos.y = y;

            if splitters.contains(&pos) {
                // Spawn right branch (starts from this splitter row)
                stack.push(IVec2::new(pos.x + 1, pos.y));

                // Current beam deflects left
                pos.x -= 1;
            }
        }

        // This beam path is finished
        beam_count += 1;
    }

    beam_count
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

    //draw(input.lines().next()?.len(), max_y + 1, &beams, &splitters);

    Some(splits.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    /*
    let mut start = IVec2::ZERO;
    let mut splitters: HashSet<IVec2> = HashSet::new();
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => start = IVec2::new(x as i32, y as i32),
                '^' => {
                    splitters.insert(IVec2::new(x as i32, y as i32));
                }
                _ => {}
            }
        }

        max_y = y;
    }

    let beams = fire_quantum_beam_iterative(start, max_y as i32, &splitters);

    Some(beams as u64)
    */
    None
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
        assert_eq!(result, None);
    }
}
