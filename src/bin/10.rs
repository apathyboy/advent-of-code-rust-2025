advent_of_code::solution!(10);

fn combinations_with_repetition<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    fn helper<T: Clone>(
        items: &[T],
        k: usize,
        start: usize,
        current: &mut Vec<T>,
        result: &mut Vec<Vec<T>>,
    ) {
        if current.len() == k {
            result.push(current.clone());
            return;
        }

        // allow picking the same index again (i passed as `start` for recursion)
        for i in start..items.len() {
            current.push(items[i].clone());
            helper(items, k, i, current, result);
            current.pop();
        }
    }

    let mut result = Vec::new();
    let mut current = Vec::with_capacity(k);
    helper(items, k, 0, &mut current, &mut result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut mins: Vec<u64> = Vec::new();
    // read in input line by line
    // - each line has
    //   - desired indicator lights ex. [.##.] - idea: convert to binary 0110
    //   - wiring schematics ex. (1,3) - idea: convert to binary 0101
    //   - joltages ex. {3,5,4,7}
    for line in input.lines() {
        let mut indicator_diagram = 0;
        let mut length = 0;
        let mut schematics: Vec<(u32, &str)> = Vec::new();

        for segment in line.split(' ') {
            match segment.chars().next().unwrap() {
                '[' => {
                    let t = &segment[1..segment.len() - 1]
                        .chars()
                        .map(|c| if c == '.' { '0' } else { '1' })
                        .collect::<String>();

                    length = (segment.len() - 2) as u32 - 1;
                    indicator_diagram = u32::from_str_radix(&t, 2).expect("Not a binary number");
                }
                '(' => {
                    let schematic = &segment[1..segment.len() - 1]
                        .split(',')
                        .fold(0, |acc, x| acc | 1 << length - x.parse::<u32>().unwrap());
                    schematics.push((*schematic, segment));
                }
                '{' => {}
                _ => {}
            }
        }

        for i in 1..100 {
            let combos = combinations_with_repetition(&schematics, i);

            let mut found = false;
            for combo in combos {
                let check = combo.iter().fold(0, |acc, &x| acc ^ x.0);

                if indicator_diagram == check {
                    found = true;
                    break;
                }
            }

            if found {
                mins.push(i as u64);
                break;
            }
        }
    }

    Some(mins.iter().sum())
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
