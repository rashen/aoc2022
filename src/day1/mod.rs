use std::fs;

pub fn main() {
    let contents = fs::read_to_string("src/day1/input.txt").expect("Missing file");
    println!("Task1: {}", calculate_max_calories(contents.clone()));
    println!("Task2: {}", calculate_top_three_calories(contents));
}

fn calculate_max_calories(input: String) -> i32 {
    *get_sorted_calories(input).last().unwrap()
}

fn calculate_top_three_calories(input: String) -> i32 {
    get_sorted_calories(input)
        .iter()
        .rev()
        .take(3)
        .fold(0, |acc, val| acc + val)
}

fn get_sorted_calories(input: String) -> Vec<i32> {
    let mut elfs = vec![0];
    for c in input.split('\n').into_iter() {
        if c.is_empty() {
            // Seperator, beginning of new elf storage
            elfs.push(0);
            continue;
        }

        if let Ok(val) = c.parse::<i32>() {
            *elfs.last_mut().unwrap() += val;
        }
    }
    elfs.sort();
    elfs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000")
    }

    #[test]
    fn task1() {
        let expected_outcome = 24000;
        assert_eq!(calculate_max_calories(get_test_input()), expected_outcome);
    }

    #[test]
    fn task2() {
        let expected_outcome = 24000 + 11000 + 10000;
        assert_eq!(
            calculate_top_three_calories(get_test_input()),
            expected_outcome
        );
    }
}
