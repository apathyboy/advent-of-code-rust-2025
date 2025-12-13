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

fn get_binary_buttons(buttons: &[Vec<u32>]) -> Vec<u32> {
    buttons
        .iter()
        .map(|b| b.iter().map(|n| 1u32 << n).sum())
        .collect()
}

// I gave up, went to Reddit and found this hint:
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
// > find all possible sets of buttons you can push so that the remaining voltages are even, and divide by 2 and recurse.
fn fewest_joltage_presses(machine: &Machine) -> usize {
    let binary_buttons = get_binary_buttons(&machine.buttons);
    let subset_xors: Vec<_> = subsets(&binary_buttons)
        .iter()
        .map(|subset| (subset.to_owned(), subset.iter().fold(0, |a, &b| a ^ b)))
        .collect();
    fewest_joltage_presses_recur(&subset_xors, &machine.joltages).unwrap()
}

fn fewest_joltage_presses_recur(
    subset_xors: &[(Vec<u32>, u32)],
    joltages: &[i32],
) -> Option<usize> {
    if joltages.iter().all(|&j| j == 0) {
        return Some(0);
    }
    let binary_joltages = get_binary_joltages(joltages);
    let mut best = None;
    for (subset, xor) in subset_xors {
        if *xor == binary_joltages {
            let new_joltages = get_new_joltages(joltages, &subset);
            if new_joltages.iter().all(|&j| j >= 0) {
                let press_count = fewest_joltage_presses_recur(subset_xors, &new_joltages)
                    .map(|c| subset.len() + 2 * c);
                best = best.min(press_count).or(best).or(press_count);
            }
        }
    }
    best
}

fn get_new_joltages(joltages: &[i32], subset: &[u32]) -> Vec<i32> {
    let mut new_joltages = Vec::new();
    let mut mask = 1;
    for &joltage in joltages {
        new_joltages.push((joltage - subset.iter().filter(|&b| b & mask != 0).count() as i32) / 2);
        mask <<= 1;
    }
    new_joltages
}

fn get_binary_joltages(joltages: &[i32]) -> u32 {
    joltages
        .iter()
        .enumerate()
        .map(|(i, j)| ((1 << i) * (j % 2)) as u32)
        .sum()
}

struct Machine {
    _lights: u32,
    buttons: Vec<Vec<u32>>,
    joltages: Vec<i32>,
}

fn subsets<T: Copy>(set: &[T]) -> Vec<Vec<T>> {
    let mut subsets: Vec<Vec<T>> = Vec::new();
    for count in 0..=set.len() {
        subsets.extend(get_combinations(set, count));
    }
    subsets
}

fn get_combinations<T: Copy>(set: &[T], count: usize) -> Vec<Vec<T>> {
    if count == 0 {
        vec![Vec::new()]
    } else {
        set[..set.len() - count + 1]
            .iter()
            .enumerate()
            .flat_map(|(i, &t)| {
                get_combinations(&set[i + 1..], count - 1)
                    .iter()
                    .map(|c| {
                        let mut c1 = c.clone();
                        c1.push(t);
                        c1
                    })
                    .collect::<Vec<Vec<T>>>()
            })
            .collect()
    }
}

fn parse_line(s: &str) -> Machine {
    let mut state: u32 = 0;
    let mut buttons: Vec<Vec<u32>> = Vec::new();
    let mut joltages: Vec<i32> = Vec::new();
    for part in s.split_whitespace() {
        let first_char = part.chars().next().unwrap();
        let middle = &part[1..part.len() - 1];
        match first_char {
            '[' => {
                state = middle
                    .chars()
                    .enumerate()
                    .filter(|&(_, ch)| ch == '#')
                    .map(|(i, _)| 1 << i)
                    .sum();
            }
            '(' => {
                buttons.push(parse_nums(middle));
            }
            '{' => joltages = parse_nums(middle).iter().map(|&j| j as i32).collect(),
            _ => unreachable!(),
        }
    }
    Machine {
        _lights: state,
        buttons,
        joltages,
    }
}

fn parse_nums(s: &str) -> Vec<u32> {
    s.split(',').map(|p| p.parse().unwrap()).collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines: Vec<Machine> = input.lines().map(|l| parse_line(&l)).collect();
    Some(machines.iter().map(fewest_joltage_presses).sum::<usize>() as u64)
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
        assert_eq!(result, Some(33));
    }
}
