advent_of_code::solution!(3);

fn find_largest_number(numbers: &str) -> u32 {
    let numbers2: Vec<u32> = numbers.chars().map(|n| n.to_digit(10).unwrap()).collect();

    // find the first largest number
    let mut largest = 0;
    for &number in &numbers2[..numbers2.len() - 1] {
        if number > largest {
            largest = number;
        }
    }

    // find the second largest number that appears after the first largest number
    let mut second_largest = 0;
    let mut found_largest = false;
    for &number in &numbers2 {
        if !found_largest && number == largest {
            found_largest = true;
            continue;
        }
        if found_largest && number > second_largest {
            second_largest = number;
        }
    }

    //let num = largest * 10 + second_largest;
    //println!(
    //    "str: {}, largest: {}, second_largest: {}, num: {}",
    //    numbers, largest, second_largest, num
    //);
    largest * 10 + second_largest
}

pub fn part_one(input: &str) -> Option<u64> {
    let total_joltage: u32 = input.lines().map(find_largest_number).sum();

    Some(total_joltage as u64)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
