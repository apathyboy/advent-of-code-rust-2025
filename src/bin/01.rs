advent_of_code::solution!(1);

fn parse(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|w| match w.as_bytes()[0] {
            b'L' => -w[1..].parse::<i16>().unwrap(),
            b'R' => w[1..].parse::<i16>().unwrap(),
            _ => 0,
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial_position = 50;
    let password = parse(input)
        .iter()
        .filter(|&clicks| {
            dial_position = (dial_position + clicks).rem_euclid(100);

            dial_position == 0
        })
        .count();

    Some(password as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial_position = 50;
    let password: i16 = parse(input)
        .iter()
        .map(|&clicks| {
            let clicked_zeros = clicks.abs() / 100
                + i16::from(
                    dial_position == 0
                        || dial_position + clicks % 100 < 0
                        || dial_position + clicks % 100 > 100,
                );

            dial_position = (dial_position + clicks).rem_euclid(100);

            clicked_zeros
        })
        .sum();

    Some(password as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
