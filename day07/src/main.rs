use aoc_utils::PuzzleInput;
const DAY: u8 = 7;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> i32 {
    let start_positions = input.convert_to_ints::<i32>();
    let min_position = *start_positions.iter().min().unwrap();
    let max_position = *start_positions.iter().max().unwrap();

    (min_position..=max_position)
        .map(|target_position| {
            start_positions
                .iter()
                .map(|pos| target_position - pos)
                .map(|diff| diff.abs())
                .sum()
        })
        .min()
        .unwrap()
}

fn solve_b(input: &PuzzleInput) -> i32 {
    let start_positions = input.convert_to_ints::<i32>();
    let min_position = *start_positions.iter().min().unwrap();
    let max_position = *start_positions.iter().max().unwrap();

    (min_position..=max_position)
        .map(|target_position| {
            start_positions
                .iter()
                .map(|pos| target_position - pos)
                .map(|diff| diff.abs())
                .map(|diff| diff * (diff + 1) / 2) // Gaussian sum formula
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 37);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 168);
    }
}
