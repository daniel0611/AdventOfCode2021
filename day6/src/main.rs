use aoc_utils::PuzzleInput;
const DAY: u8 = 6;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn simulate_fish(input: &PuzzleInput, days: u16) -> u64 {
    // Maps the internal state (the day) of a fish (as index) to the count of how many fish are in that state
    let mut fish_counts = [0; 9].to_vec();

    let initial_fish_states: Vec<u8> = input
        .raw_input
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    for fish in initial_fish_states {
        // E.g. this fish is in the first state (0) => increase count at index 0, etc.
        fish_counts[fish as usize] += 1;
    }

    for _ in 1..=days {
        // Because we don't track every fish individually, just how many are in which state, this is now easy
        let fish_with_state_zero = fish_counts[0];

        // remove these fish from day 0 and re-add them at day 6 (7 because we will reduce it by one with the drain call)
        fish_counts[7] += fish_with_state_zero;
        fish_counts.drain(0..1);

        // Add the newly born fish
        fish_counts.push(fish_with_state_zero);
    }

    fish_counts.iter().sum()
}

fn solve_a(input: &PuzzleInput) -> u64 {
    simulate_fish(input, 80)
}

fn solve_b(input: &PuzzleInput) -> u64 {
    simulate_fish(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 5934);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 26984457539);
    }
}
