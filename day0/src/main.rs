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
