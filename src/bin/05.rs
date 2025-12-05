advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut id_ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    for line in input.lines() {
        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].trim().parse().ok()?;
            let end: u64 = parts[1].trim().parse().ok()?;
            id_ranges.push((start, end));
        } else if line.len() > 0 {
            let id: u64 = line.trim().parse().ok()?;
            ids.push(id);
        }
    }

    let mut fresh_ids: Vec<u64> = Vec::new();

    for id in ids {
        for (start, end) in &id_ranges {
            if id >= *start && id <= *end {
                fresh_ids.push(id);
                break;
            }
        }
    }

    Some(fresh_ids.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut id_ranges: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].trim().parse().ok()?;
            let end: u64 = parts[1].trim().parse().ok()?;
            id_ranges.push((start, end));
        }
    }

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    id_ranges.sort_by_key(|k| k.0);
    for range in id_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    let mut fresh_count: u64 = 0;
    for range in merged_ranges {
        fresh_count += range.1 - range.0 + 1;
    }

    Some(fresh_count)
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
        assert_eq!(result, Some(14));
    }
}
