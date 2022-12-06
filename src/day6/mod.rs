use std::{collections::HashSet, fs};

pub fn main() {
    let input = fs::read_to_string("src/day6/input.txt").unwrap();
    println!("Task1: {}", find_start_of_packet(&input));
    println!("Task1: {}", find_start_of_msg(&input));
}

fn find_start_of_packet(signal: &str) -> usize {
    find_non_repeating_pattern(signal, 4).unwrap_or(0)
}
fn find_start_of_msg(signal: &str) -> usize {
    find_non_repeating_pattern(signal, 14).unwrap_or(0)
}

fn find_non_repeating_pattern(signal: &str, pattern_length: usize) -> Option<usize> {
    for i in (pattern_length - 1)..signal.len() {
        let mut sliding_window = HashSet::<char>::with_capacity(pattern_length);
        let first_index = i + 1 - pattern_length;
        let last_index = i;
        for c in signal[first_index..=last_index].chars() {
            sliding_window.insert(c);
        }
        if sliding_window.len() == pattern_length {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(find_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(
            find_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
        assert_eq!(find_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_task2() {
        assert_eq!(find_start_of_msg("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_start_of_msg("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_start_of_msg("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_start_of_msg("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_start_of_msg("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
