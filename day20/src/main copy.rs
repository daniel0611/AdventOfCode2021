use aoc_utils::PuzzleInput;
use std::collections::HashSet;
const DAY: u8 = 20;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_input(input: &PuzzleInput) -> (Vec<bool>, HashSet<(i32, i32)>) {
    let (algorithm, grid) = input.raw_input.split_once("\n\n").unwrap();

    let algorithm = algorithm
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Invalid input"),
        })
        .collect::<Vec<_>>();

    let grid = grid
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut lit_pixels = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                lit_pixels.insert((x as i32, y as i32));
            }
        }
    }

    (algorithm, lit_pixels)
}

fn get_with_neigbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut points = vec![];

    for dy in -1..=1 {
        for dx in -1..=1 {
            let nx = x + dx;
            let ny = y + dy;
            points.push((nx, ny));
        }
    }

    points
}

fn enhance(algorithm: &Vec<bool>, lit_pixels: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_lit_pixels = HashSet::new();
    let min_y = *lit_pixels.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = *lit_pixels.iter().map(|(_, y)| y).max().unwrap() + 1;
    let min_x = *lit_pixels.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = *lit_pixels.iter().map(|(x, _)| x).max().unwrap() + 1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let algo_addr_binary = get_with_neigbors(x, y)
                .iter()
                .map(|(x, y)| match lit_pixels.contains(&(*x, *y)) {
                    true => '1',
                    false => '0',
                })
                .collect::<String>();

            let algo_addr = usize::from_str_radix(&algo_addr_binary, 2).unwrap();
            let lit_value = algorithm[algo_addr];

            if lit_value {
                new_lit_pixels.insert((x, y));
            }
        }
    }

    new_lit_pixels
}

fn solve_a(input: &PuzzleInput) -> usize {
    let (algorithm, mut lit_pixels) = parse_input(input);

    for _ in 0..2 {
        lit_pixels = enhance(&algorithm, &lit_pixels);
    }

    lit_pixels.len()
}

fn solve_b(input: &PuzzleInput) -> usize {
    input.lines().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
    \n\
    #..#.\n\
    #....\n\
    ##..#\n\
    ..#..\n\
    ..###";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 35);
    }

    // #[test]
    // fn test_solve_b() {
    //     assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 0);
    // }
}
