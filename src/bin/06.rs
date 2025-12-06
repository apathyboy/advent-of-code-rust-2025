advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let terms = input
        .lines()
        .take(input.lines().count() - 1)
        .map(|lines| {
            lines
                .split_whitespace()
                .map(|num| num.parse::<u64>().ok())
                .collect::<Option<Vec<u64>>>()
        })
        .collect::<Option<Vec<Vec<u64>>>>()?;

    let operators = input
        .lines()
        .last()?
        .split_whitespace()
        .map(|op| op.trim())
        .collect::<Vec<&str>>();

    let mut result = 0;

    for i in 0..operators.len() {
        let mut answer = 0;
        if operators[i] == "+" {
            for term in &terms {
                answer += term[i];
            }
        }
        if operators[i] == "*" {
            answer = 1;
            for term in &terms {
                answer *= term[i];
            }
        }

        result += answer;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Collect all lines once
    let mut rows: Vec<&str> = input.lines().collect();

    // Last line is the operators
    let operators = rows.pop()?; // returns None if there were no lines

    let width = operators.len();
    if width == 0 || rows.is_empty() {
        return None;
    }

    // Precompute the column values as numbers
    // terms[i] corresponds to column i (same as your terms3[i])
    let terms: Vec<Option<u64>> = (0..width)
        .map(|col| {
            // Build the vertical string for this column
            let mut col_str = String::with_capacity(rows.len());
            for &row in &rows {
                let bytes = row.as_bytes();
                if let Some(&b) = bytes.get(col) {
                    col_str.push(b as char);
                }
            }

            let trimmed = col_str.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed.parse::<u64>().ok()
            }
        })
        .collect();

    // Evaluate from right to left
    let mut result = 0u64;
    let mut add_acc = 0u64;
    let mut mul_acc = 1u64;

    // Zip the terms with the operators and iterate in reverse
    for (maybe_val, op) in terms.iter().zip(operators.bytes()).rev() {
        if let Some(val) = maybe_val {
            add_acc += val;
            mul_acc *= val;
        }

        match op {
            b'+' => {
                result += add_acc;
                add_acc = 0;
                mul_acc = 1;
            }
            b'*' => {
                result += mul_acc;
                add_acc = 0;
                mul_acc = 1;
            }
            _ => {}
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
