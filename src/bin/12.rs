advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    for line in input.lines() {
        if !line.contains("x") {
            continue;
        }

        let (size, regions) = line.split_once(": ").expect("region expected");

        let area = size
            .split("x")
            .filter_map(|n| n.parse::<u64>().ok())
            .fold(1, |acc, x| acc * x);

        let size = regions
            .split(" ")
            .map(|n| n.parse::<u64>().unwrap() * 8)
            .sum::<u64>();

        if size < area {
            count += 1;
        }
    }

    Some(count)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
