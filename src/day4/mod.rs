use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();
    println!("Task1: {}", find_fully_overlapping_ranges(&input));
    println!("Task2: {}", find_partially_overlapping_ranges(&input));
}

fn find_fully_overlapping_ranges(input: &str) -> i32 {
    let mut overlapping_ranges = 0;
    for line in input.split_ascii_whitespace() {
        if let Some((lhs, rhs)) = parse_line(line) {
            let overlap = lhs & rhs;
            if lhs == overlap || rhs == overlap {
                overlapping_ranges += 1;
            }
        }
    }
    overlapping_ranges
}

fn find_partially_overlapping_ranges(input: &str) -> i32 {
    let mut overlapping_ranges = 0;
    for line in input.split_ascii_whitespace() {
        if let Some((lhs, rhs)) = parse_line(line) {
            let full_range = lhs | rhs;
            if full_range != rhs ^ lhs {
                overlapping_ranges += 1;
            }
        }
    }
    overlapping_ranges
}

fn parse_line(input: &str) -> Option<(u128, u128)> {
    let (lhs, rhs) = input.split_once(',')?;

    let first_range = parse_range(lhs)?;
    let second_range = parse_range(rhs)?;
    return Some((first_range, second_range));
}

fn parse_range(input: &str) -> Option<u128> {
    let (from, to) = input.split_once('-')?;
    let from = from.parse::<u128>().ok()?;
    let to = to.parse::<u128>().ok()?;

    let mut output = 0_u128;
    for i in from..=to {
        output = output | 1 << i - 1;
    }
    return Some(output);
}

#[cfg(test)]
mod tests {

    use super::*;
    fn get_test_input() -> &'static str {
        "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("2-4"), Some(14));
        assert_eq!(parse_range("6-8"), Some(224));
    }

    #[test]
    fn test_fully_overlapping_ranges() {
        let input = get_test_input();
        assert_eq!(find_fully_overlapping_ranges(input), 2);
    }

    #[test]
    fn test_partially_overlapping_ranges() {
        let input = get_test_input();
        assert_eq!(find_partially_overlapping_ranges(input), 4);
    }
}
