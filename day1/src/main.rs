use aoc_utils::PuzzleInput;
const DAY: u8 = 1;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> usize {
    let lines = input.convert_to_ints();

    let mut count = 0;
    for idx in 1..lines.len() {
        // Only where it increased
        if lines[idx] > lines[idx - 1] {
            count += 1;
        }
    }

    count
}

fn solve_b(input: &PuzzleInput) -> usize {
    let lines = input.convert_to_ints();
    let mut sliding_window_sums = vec![];

    for idx in 0..(lines.len() - 2) {
        let sum = lines[idx] + lines[idx + 1] + lines[idx + 2];
        sliding_window_sums.push(sum);
    }

    let mut increase_count = 0;
    for idx in 1..sliding_window_sums.len() {
        // Only where it increased
        if sliding_window_sums[idx] > sliding_window_sums[idx - 1] {
            increase_count += 1;
        }
    }

    increase_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        let input = PuzzleInput::new(EXAMPLE_INPUT.to_string());
        assert_eq!(solve_a(&input), 7);
    }

    #[test]
    fn test_solve_b() {
        let input = PuzzleInput::new(EXAMPLE_INPUT.to_string());
        assert_eq!(solve_b(&input), 5);
    }
}
