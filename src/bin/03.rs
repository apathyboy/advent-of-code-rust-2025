advent_of_code::solution!(3);

fn max_digit_subsequence(s: &str, k: usize) -> Option<String> {
    let n = s.len();
    if k > n {
        return None;
    }

    let mut stack: Vec<u8> = Vec::with_capacity(n);
    let mut to_remove = n - k; // how many digits we are allowed to drop

    for b in s.bytes() {
        // While the stack's last digit is smaller than the current digit
        // and we still can remove digits, pop to make number larger.
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < b {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(b);
    }

    // We may still have more digits than needed if we didn't remove enough.
    stack.truncate(k);

    // Safe because input is ASCII digits.
    Some(String::from_utf8(stack).unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    let total_joltage: u64 = input
        .lines()
        .map(|line| {
            if let Some(subseq) = max_digit_subsequence(line, 2) {
                subseq.parse::<u64>().unwrap()
            } else {
                0
            }
        })
        .sum();

    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total_joltage: u64 = input
        .lines()
        .map(|line| {
            if let Some(subseq) = max_digit_subsequence(line, 12) {
                subseq.parse::<u64>().unwrap()
            } else {
                0
            }
        })
        .sum();

    Some(total_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
