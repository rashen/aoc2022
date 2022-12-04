use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/day3/input.txt")
        .expect("Missing file")
        .split_ascii_whitespace()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    println!(
        "Task1: {}",
        calculate_sum_of_priorites_of_duplicates(&input)
    );
    println!("Task2: {}", calculate_sum_of_priorites_of_badges(&input));
}

fn calculate_sum_of_priorites_of_duplicates(input: &[String]) -> i32 {
    let duplicates = find_duplicates(input);
    calculate_priority(&duplicates)
}

fn calculate_sum_of_priorites_of_badges(input: &[String]) -> i32 {
    let badges = find_badges(input);
    calculate_priority(&badges)
}

fn find_duplicates(input: &[String]) -> Vec<char> {
    let mut output = Vec::new();
    'outer: for rucksack in input {
        let len = rucksack.len();
        let (first_compartment, second_compartment) = rucksack.split_at(len / 2);
        for c in first_compartment.chars() {
            if second_compartment.contains(c) {
                output.push(c);
                continue 'outer;
            }
        }
    }
    output
}

fn find_badges(input: &[String]) -> Vec<char> {
    let mut output = Vec::new();
    'outer: for i in (0..input.len()).step_by(3) {
        let first_rucksack = &input[i];
        let second_rucksack = &input[i + 1];
        let third_rucksack = &input[i + 2];
        for c in first_rucksack.chars() {
            if second_rucksack.contains(c) && third_rucksack.contains(c) {
                output.push(c);
                continue 'outer;
            }
        }
    }
    output
}

fn calculate_priority(items: &[char]) -> i32 {
    let mut priority_sum = 0;
    for e in items {
        if e.is_uppercase() {
            let priority = *e as i32 - 'A' as i32 + 27;
            priority_sum += priority;
        } else {
            let priority = *e as i32 - 'a' as i32 + 1;
            priority_sum += priority;
        }
    }
    priority_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_duplicates() {
        let input = get_test_input();
        assert_eq!(find_duplicates(&input), vec!['p', 'L', 'P', 'v', 't', 's']);
    }

    #[test]
    fn test_priority() {
        assert_eq!(calculate_priority(&vec!['a']), 1);
        assert_eq!(calculate_priority(&vec!['b']), 2);
        assert_eq!(calculate_priority(&vec!['z']), 26);
        assert_eq!(calculate_priority(&vec!['A']), 27);
        assert_eq!(calculate_priority(&vec!['Z']), 52);
        assert_eq!(calculate_priority(&vec!['a', 'Z']), 1 + 52);
    }

    #[test]
    fn task1() {
        let input = get_test_input();
        assert_eq!(calculate_sum_of_priorites_of_duplicates(&input), 157);
    }

    #[test]
    fn task2() {
        let input = get_test_input();
        assert_eq!(calculate_sum_of_priorites_of_badges(&input), 70);
    }
}
