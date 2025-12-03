advent_of_code::solution!(2);

const POW10: [u64; 11] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
];

fn num_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

fn has_pattern_repeated_twice(n: u64) -> bool {
    let len = num_digits(n);

    if !len.is_multiple_of(2) {
        return false;
    }

    let half = len / 2;
    let pow10_half = POW10[half as usize];

    let hi = n / pow10_half; // first half
    let lo = n % pow10_half; // second half

    hi == lo
}

fn has_repeating_pattern(n: u64) -> bool {
    let len = num_digits(n) as usize;
    let pow10_len = POW10[len]; // assuming POW10[i] = 10^i as u64

    for sub_len in 1..=len / 2 {
        if !len.is_multiple_of(sub_len) {
            continue;
        }

        let pow10_sub = POW10[sub_len];

        // First sub_len digits as the pattern
        let pattern = n / POW10[len - sub_len];

        // Geometric series: 1 + r + r^2 + ... + r^(k-1),
        // where r = 10^sub_len and k = len / sub_len.
        let multiplier = (pow10_len - 1) / (pow10_sub - 1);

        // Avoid overflow just in case (even though any valid n must fit in u64)
        if let Some(acc) = pattern.checked_mul(multiplier) {
            if acc == n {
                return true;
            }
        }
    }

    false
}

fn parse_range(range: &str) -> Option<(u64, u64)> {
    let mut parts = range.split('-').map(|s| s.parse::<u64>());
    let start = parts.next()?.ok()?;
    let end = parts.next()?.ok()?;
    Some((start, end))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_sum: u64 = 0;

    for range in input.trim().split(',') {
        let (start, end) = parse_range(range)?;

        for n in start..=end {
            if has_pattern_repeated_twice(n) {
                invalid_sum += n;
            }
        }
    }

    Some(invalid_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_sum: u64 = 0;

    for range in input.trim().split(',') {
        let (start, end) = parse_range(range)?;

        for n in start..=end {
            if has_repeating_pattern(n) {
                invalid_sum += n;
            }
        }
    }

    Some(invalid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
