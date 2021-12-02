use aoc_utils::PuzzleInput;
const DAY: u8 = 2;

enum Direction {
    Forward,
    Down,
    Up
}

struct SubmarineCommand {
    direction: Direction,
    value: i32
}

impl SubmarineCommand {
    fn prase(line: &str) -> SubmarineCommand {
        let mut parts = line.split_whitespace();
        let direction = match parts.next().unwrap() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Unknown direction: {}", parts.next().unwrap())
        };
        let value = parts.next().unwrap().parse::<i32>().unwrap();
        SubmarineCommand { direction, value }
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;

    let commands: Vec<SubmarineCommand> = input.lines().iter().map(|line| SubmarineCommand::prase(line)).collect();
    for command in commands {
        match command.direction {
            Direction::Forward => horizontal += command.value,
            Direction::Down => depth += command.value,
            Direction::Up => depth -= command.value
        }
    }

    horizontal * depth
}

fn solve_b(input: &PuzzleInput) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    let commands: Vec<SubmarineCommand> = input.lines().iter().map(|line| SubmarineCommand::prase(line)).collect();
    for command in commands {
        match command.direction {
            Direction::Forward => {
                horizontal += command.value;
                depth += command.value * aim;
            },
            Direction::Down => aim += command.value,
            Direction::Up => aim -= command.value
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "forward 5\n\
    down 5\n\
    forward 8\n\
    up 3\n\
    down 8\n\
    forward 2";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        let input = PuzzleInput::new(EXAMPLE_INPUT);
        assert_eq!(150, solve_a(&input))
    }

    #[test]
    fn test_solve_b() {
        let input = PuzzleInput::new(EXAMPLE_INPUT);
        assert_eq!(900, solve_b(&input))
    }
}
