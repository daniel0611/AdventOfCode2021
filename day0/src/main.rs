use aoc_utils::PuzzleInput;
const DAY: u8 = 0;

fn main() {
    println!("A: {} lines", solve_a());
    println!("B: {} lines", solve_b());
}

fn solve_a() -> usize {
    let input = PuzzleInput::get_input_a(DAY);
    let lines = input.lines();

    for line in &lines {
        println!("{}", line);
    }

    lines.len()
}

fn solve_b() -> usize {
    PuzzleInput::get_input_b(DAY).lines().len()
}

#[cfg(test)]
mod tests {
    // solve_a should return 2
    #[test]
    fn test_solve_a() {
        assert_eq!(super::solve_a(), 2);
    }

    // solve_b should return 1
    #[test]
    fn test_solve_b() {
        assert_eq!(super::solve_b(), 1);
    }
}
