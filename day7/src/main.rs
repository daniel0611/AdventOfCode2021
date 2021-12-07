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

    let mut costs = vec![];

    for target_position in min_position..=max_position {
        let mut required_fuel = 0;

        for pos in start_positions.iter() {
            required_fuel += (target_position - pos).abs()
        }
    
        costs.push(required_fuel);
    }

    *costs.iter().min().unwrap()
}

fn solve_b(input: &PuzzleInput) -> i32 {
    let start_positions = input.convert_to_ints::<i32>();
    let min_position = *start_positions.iter().min().unwrap();
    let max_position = *start_positions.iter().max().unwrap();

    let mut costs = vec![];

    for target_position in min_position..=max_position {
        let mut required_fuel = 0;

        for pos in start_positions.iter() {
            let diff = (target_position - pos).abs();
            for f in 1..=diff {
                required_fuel += f;
            }
        }
    
        costs.push(required_fuel);
    }

    *costs.iter().min().unwrap()
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
