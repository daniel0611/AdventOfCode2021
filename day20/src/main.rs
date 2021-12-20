use aoc_utils::PuzzleInput;
use std::collections::HashMap;
const DAY: u8 = 20;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

type Point = (i32, i32);

fn parse_input(input: &PuzzleInput) -> (Vec<bool>, HashMap<Point, bool>) {
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

    let mut pixels = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let lit = *pixel == '#';
            pixels.insert((x as i32, y as i32), lit);
        }
    }

    (algorithm, pixels)
}

fn get_with_neigbors(x: i32, y: i32) -> Vec<Point> {
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

fn enhance(
    algorithm: &[bool],
    pixels: &HashMap<Point, bool>,
    background_is_lit: bool,
) -> HashMap<Point, bool> {
    let mut new_pixels = HashMap::new();

    let min_x = pixels.keys().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = pixels.keys().map(|(x, _)| x).max().unwrap() + 1;
    let min_y = pixels.keys().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = pixels.keys().map(|(_, y)| y).max().unwrap() + 1;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let algo_addr_binary = get_with_neigbors(x, y)
                .iter()
                .map(|p| pixels.get(p))
                .map(|p| match p {
                    Some(true) => '1',
                    Some(false) => '0',
                    None => {
                        if background_is_lit {
                            '1'
                        } else {
                            '0'
                        }
                    }
                })
                .collect::<String>();

            let algo_addr = usize::from_str_radix(&algo_addr_binary, 2).unwrap();
            let lit_value = algorithm[algo_addr];

            new_pixels.insert((x, y), lit_value);
        }
    }

    new_pixels
}

fn solve_a(input: &PuzzleInput) -> usize {
    let (algorithm, mut pixels) = parse_input(input);

    for i in 0..2 {
        let bg = if algorithm[0] { i % 2 == 1 } else { false };
        pixels = enhance(&algorithm, &pixels, bg);
    }

    pixels.values().filter(|p| **p).count()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let (algorithm, mut pixels) = parse_input(input);

    for i in 0..50 {
        let bg = if algorithm[0] { i % 2 == 1 } else { false };
        pixels = enhance(&algorithm, &pixels, bg);
    }

    pixels.values().filter(|p| **p).count()
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
        // solve_b(&input); too slow
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 35);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 3351);
    }
}
