use std::fs;

// [N]         [C]     [Z]
// [Q] [G]     [V]     [S]         [V]
// [L] [C]     [M]     [T]     [W] [L]
// [S] [H]     [L]     [C] [D] [H] [S]
// [C] [V] [F] [D]     [D] [B] [Q] [F]
// [Z] [T] [Z] [T] [C] [J] [G] [S] [Q]
// [P] [P] [C] [W] [W] [F] [W] [J] [C]
// [T] [L] [D] [G] [P] [P] [V] [N] [R]
//  1   2   3   4   5   6   7   8   9

pub fn main() {
    let start_positions = vec![
        vec!['T', 'P', 'Z', 'C', 'S', 'L', 'Q', 'N'],
        vec!['L', 'P', 'T', 'V', 'H', 'C', 'G'],
        vec!['D', 'C', 'Z', 'F'],
        vec!['G', 'W', 'T', 'D', 'L', 'M', 'V', 'C'],
        vec!['P', 'W', 'C'],
        vec!['P', 'F', 'J', 'D', 'C', 'T', 'S', 'Z'],
        vec!['V', 'W', 'G', 'B', 'D'],
        vec!['N', 'J', 'S', 'Q', 'H', 'W'],
        vec!['R', 'C', 'Q', 'F', 'S', 'L', 'V'],
    ];
    let input = fs::read_to_string("src/day5/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let end_positions = calculate_crate_positions(&input, &start_positions);
    println!("Task1: {}", top_crates(&end_positions));
    println!(
        "Task2: {}",
        top_crates(&calculate_crate_positions_new_and_improved(
            &input,
            &start_positions
        ))
    );
}

fn calculate_crate_positions(input: &Vec<String>, start: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut crates = start.clone();
    for line in input {
        let line: Vec<&str> = line.split(' ').collect();
        if line.len() < 6 {
            continue; // Incomplete instruction
        }

        let iterations = line[1].parse::<usize>().unwrap();
        let start = line[3].parse::<usize>().unwrap();
        let end = line[5].parse::<usize>().unwrap();

        for _ in 0..iterations {
            let crate_to_move = crates[start - 1].pop().unwrap();
            crates[end - 1].push(crate_to_move);
        }
    }
    crates
}

fn calculate_crate_positions_new_and_improved(
    input: &Vec<String>,
    start: &Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    let mut crates = start.clone();
    for line in input {
        let line: Vec<&str> = line.split(' ').collect();
        if line.len() < 6 {
            continue; // Incomplete instruction
        }

        let iterations = line[1].parse::<usize>().unwrap();
        let start = line[3].parse::<usize>().unwrap();
        let end = line[5].parse::<usize>().unwrap();

        let mut crates_to_move = Vec::new();
        for _ in 0..iterations {
            crates_to_move.push(crates[start - 1].pop().unwrap());
        }
        for c in crates_to_move.iter().rev() {
            crates[end - 1].push(*c);
        }
    }
    crates
}

fn top_crates(crates: &Vec<Vec<char>>) -> String {
    crates
        .iter()
        .fold(String::new(), |acc, c| acc + &c.last().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_start_positions() -> Vec<Vec<char>> {
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
    }

    fn get_test_input() -> Vec<String> {
        vec![
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_calculate_crate_positions() {
        let output = calculate_crate_positions(&get_test_input(), &get_start_positions());
        assert_eq!(output, vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']]);
    }

    #[test]
    fn test_task1() {
        let output = calculate_crate_positions(&get_test_input(), &get_start_positions());
        assert_eq!(top_crates(&output), "CMZ");
    }
}
