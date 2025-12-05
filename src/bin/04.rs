advent_of_code::solution!(4);

fn is_accessible(map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // count all 8 directions for "@"
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // if fewer than 4 direcitons have "@" return true else false
    let mut count = 0;
    for (dx, dy) in directions.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0
            && ny >= 0
            && (nx as usize) < map.len()
            && (ny as usize) < map[0].len()
            && map[ny as usize][nx as usize] == '@'
        {
            count += 1;
        }
    }
    count < 4
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = map[0].len();
    let height = map.len();

    let mut accessible_positions = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] != '.' && is_accessible(&map, x, y) {
                accessible_positions.push((x, y));
            }
        }
    }

    Some(accessible_positions.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = map[0].len();
    let height = map.len();

    let mut accessible_positions = Vec::new();

    let mut changed = true;
    while changed {
        changed = false;
        for y in 0..height {
            for x in 0..width {
                if map[y][x] != '.' && is_accessible(&map, x, y) {
                    accessible_positions.push((x, y));
                    map[y][x] = '.';
                    changed = true;
                }
            }
        }
    }

    Some(accessible_positions.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
