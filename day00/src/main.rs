use aoc_utils::PuzzleInput;
const DAY: u8 = 0;

fn main() {
    println!("A: {} lines", solve_a());
}

fn solve_a() -> usize {
    let input = PuzzleInput::get_input(DAY);
    let lines = input.lines();

    for line in &lines {
        println!("{}", line);
    }

    lines.len()
}

#[cfg(test)]
mod tests {
    // solve_a should return 2
    #[test]
    fn test_solve_a() {
        assert_eq!(super::solve_a(), 2);
    }
}
