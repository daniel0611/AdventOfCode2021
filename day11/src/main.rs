use aoc_utils::PuzzleInput;
const DAY: u8 = 11;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_grid(input: &PuzzleInput) -> Vec<Vec<u8>> {
    let lines = input.lines();
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn get_adjacent(grid: &[Vec<u8>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut adjacent = vec![];
    for ax in x.saturating_sub(1)..=x + 1 {
        for ay in y.saturating_sub(1)..=y + 1 {
            // Input, not a neighbor
            if ax == x && ay == y {
                continue;
            }
            // Overflows
            if ay >= grid.len() || ax >= grid[ay].len() {
                continue;
            }

            adjacent.push((ax, ay));
        }
    }
    adjacent
}

fn process_flashes(grid: &mut Vec<Vec<u8>>, old_flashes: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut new_flashes = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // This octopus creates a flash and hasn't flashed already
            if grid[y][x] > 9 && !old_flashes.contains(&(x, y)) {
                new_flashes.push((x, y));
                let adjacent = get_adjacent(grid, x, y);

                for (ax, ay) in adjacent.iter() {
                    grid[*ay][*ax] += 1;
                }
            }
        }
    }

    new_flashes
}

fn simulate_step(grid: &mut Vec<Vec<u8>>) -> usize {
    let mut flashes = vec![];

    for row in &mut grid.iter_mut() {
        for cell in &mut row.iter_mut() {
            *cell += 1;
        }
    }

    loop {
        let new_flashes = process_flashes(grid, &flashes);
        if new_flashes.is_empty() {
            break;
        }

        flashes.extend(new_flashes);
    }

    // Reset those octopuses that flashed to zero
    for (x, y) in flashes.iter() {
        grid[*y][*x] = 0;
    }
    flashes.len()
}

fn solve_a(input: &PuzzleInput) -> usize {
    let mut grid = parse_grid(input);
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += simulate_step(&mut grid);
    }

    flashes
}

fn solve_b(input: &PuzzleInput) -> usize {
    let mut grid = parse_grid(input);
    let mut step = 0;

    loop {
        let flashes = simulate_step(&mut grid);
        step += 1;

        if flashes == grid.len() * grid[0].len() {
            return step;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5483143223\n\
    2745854711\n\
    5264556173\n\
    6141336146\n\
    6357385478\n\
    4167524645\n\
    2176841721\n\
    6882881134\n\
    4846848554\n\
    5283751526";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 1656);
    }

    #[test]
    fn test_solve_a_step1() {
        let mut grid = parse_grid(&PuzzleInput::new(TEST_INPUT));
        simulate_step(&mut grid);
        simulate_step(&mut grid);
        let output_2 = "8807476555\n\
        5089087054\n\
        8597889608\n\
        8485769600\n\
        8700908800\n\
        6600088989\n\
        6800005943\n\
        0000007456\n\
        9000000876\n\
        8700006848";
        assert_eq!(grid, parse_grid(&PuzzleInput::new(output_2)));
    }

    #[test]
    fn test_solve_a_easy_example_step1() {
        let input = "11111\n\
        19991\n\
        19191\n\
        19991\n\
        11111";
        let mut grid = parse_grid(&PuzzleInput::new(input));
        let new_flashes_1 = simulate_step(&mut grid);

        assert_eq!(new_flashes_1, 9);
        let output_1 = "34543\n\
        40004\n\
        50005\n\
        40004\n\
        34543";
        assert_eq!(grid, parse_grid(&PuzzleInput::new(output_1)));

        simulate_step(&mut grid);
        let output_2 = "45654\n\
        51115\n\
        61116\n\
        51115\n\
        45654";
        assert_eq!(grid, parse_grid(&PuzzleInput::new(output_2)));
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 195);
    }
}
