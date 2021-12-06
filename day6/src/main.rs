use aoc_utils::PuzzleInput;
const DAY: u8 = 6;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn simulate_fish(input: &PuzzleInput, days: u16) -> usize {
    let mut fish_states: Vec<u8> = input.raw_input
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    for _ in 1..=days {
        let mut new_state = fish_states.clone();

        for i in 0..fish_states.len() {
            let v = fish_states[i];
            if v == 0 {
                new_state[i] = 6;
                new_state.push(8);
            } else {
                new_state[i] = v - 1;
            }
        }

        fish_states = new_state;
    }

    fish_states.len()
}

fn solve_a(input: &PuzzleInput) -> usize {
    simulate_fish(input, 80)
}

fn solve_b(input: &PuzzleInput) -> usize {
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
