use std::fs;

pub fn main() {
    let contents = fs::read_to_string("src/day2/input.txt").expect("Missing file");

    let strategy = contents
        .split_ascii_whitespace()
        .map(|s| s.chars().nth(0))
        .filter_map(|c| c)
        .collect::<Vec<char>>();

    assert_eq!(strategy.len(), 5000);

    println!("Task1: {}", calculate_total_score(&strategy));
    println!(
        "Task2: {}",
        calculate_score_with_correct_instructions(&strategy)
    );
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum HandSign {
    Rock = 0,
    Paper = 1,
    Sciccors = 2,
}
enum GameOutcome {
    Win,
    Lose,
    Draw,
}

impl From<i8> for HandSign {
    fn from(value: i8) -> Self {
        use HandSign::*;
        match value {
            0 => Rock,
            1 => Paper,
            2 => Sciccors,
            _ => unimplemented!(),
        }
    }
}
impl From<char> for HandSign {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => HandSign::Rock,
            'B' | 'Y' => HandSign::Paper,
            'C' | 'Z' => HandSign::Sciccors,
            _ => unimplemented!(),
        }
    }
}

impl HandSign {
    fn beats(&self, other: &HandSign) -> GameOutcome {
        use GameOutcome::*;
        let lhs = *self as i8;
        let rhs = *other as i8;
        if lhs == rhs {
            return Draw;
        }
        if (lhs - 1).rem_euclid(3) == rhs {
            return Win;
        }
        Lose
    }
    fn score(&self) -> i32 {
        match self {
            HandSign::Rock => 1,
            HandSign::Paper => 2,
            HandSign::Sciccors => 3,
        }
    }
}
impl GameOutcome {
    fn score(&self) -> i32 {
        match self {
            GameOutcome::Win => 6,
            GameOutcome::Lose => 0,
            GameOutcome::Draw => 3,
        }
    }
}
impl From<char> for GameOutcome {
    fn from(value: char) -> Self {
        use GameOutcome::*;
        match value {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => unimplemented!(),
        }
    }
}

fn get_strategy(lhs: HandSign, rhs: GameOutcome) -> HandSign {
    let lhs = lhs as i8;
    match rhs {
        GameOutcome::Win => (lhs + 1).rem_euclid(3).into(),
        GameOutcome::Lose => (lhs - 1).rem_euclid(3).into(),
        GameOutcome::Draw => lhs.into(),
    }
}

fn calculate_total_score(strategy: &[char]) -> i32 {
    let mut score = 0;
    for i in (0..strategy.len()).step_by(2) {
        let lhs: HandSign = strategy[i].into();
        let rhs: HandSign = strategy[i + 1].into();
        score += rhs.score();
        score += rhs.beats(&lhs).score();
    }

    score
}

fn calculate_score_with_correct_instructions(strategy: &[char]) -> i32 {
    let mut score = 0;
    for i in (0..strategy.len()).step_by(2) {
        let lhs = strategy[i].into();
        let rhs = get_strategy(lhs, strategy[i + 1].into());
        score += rhs.score();
        score += rhs.beats(&lhs).score();
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<char> {
        vec!['A', 'Y', 'B', 'X', 'C', 'Z']
    }

    #[test]
    fn task1() {
        let input = get_test_input();
        assert_eq!(calculate_total_score(&input), 15);
    }

    #[test]
    fn task2() {
        let input = get_test_input();
        assert_eq!(calculate_score_with_correct_instructions(&input), 12);
    }

    #[test]
    fn test_get_strategy() {
        use GameOutcome::*;
        use HandSign::*;
        assert_eq!(get_strategy(Rock, Lose), Sciccors);
        assert_eq!(get_strategy(Rock, Draw), Rock);
        assert_eq!(get_strategy(Rock, Win), Paper);
        assert_eq!(get_strategy(Sciccors, Lose), Paper);
        assert_eq!(get_strategy(Sciccors, Draw), Sciccors);
        assert_eq!(get_strategy(Sciccors, Win), Rock);
        assert_eq!(get_strategy(Paper, Lose), Rock);
        assert_eq!(get_strategy(Paper, Draw), Paper);
        assert_eq!(get_strategy(Paper, Win), Sciccors);
    }
}
