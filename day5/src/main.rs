use aoc_utils::PuzzleInput;
use std::cmp;
const DAY: u8 = 5;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_lines(input: &PuzzleInput) -> Vec<Line> {
    input
        .lines()
        .iter()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let start = parts
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let end = parts
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Line {
                start: (start[0], start[1]),
                end: (end[0], end[1]),
            }
        })
        .collect::<Vec<Line>>()
}

fn get_grid_size(lines: &[Line]) -> (usize, usize) {
    let max_x = lines
        .iter()
        .map(|line| cmp::max(line.start.0, line.end.0))
        .max()
        .unwrap();
    let max_y = lines
        .iter()
        .map(|line| cmp::max(line.start.1, line.end.1))
        .max()
        .unwrap();
    (max_x as usize + 1, max_y as usize + 1)
}

fn count_overlaps(grid: &[Vec<u32>]) -> usize {
    let mut count = 0;
    for row in grid.iter() {
        for cell in row.iter() {
            // print!("{}", *cell);
            if *cell > 1 {
                count += 1;
            }
        }
        // println!();
    }

    count
}

fn solve_a(input: &PuzzleInput) -> usize {
    let lines = parse_lines(input);
    let (max_x, max_y) = get_grid_size(&lines);
    let mut grid = vec![vec![0u32; max_x as usize + 1]; max_y as usize + 1];

    for line in &lines {
        let (start_x, start_y) = line.start;
        let (end_x, end_y) = line.end;

        if line.start.0 != line.end.0 && line.start.1 != line.end.1 {
            // Horizontal line, ignored in task a
            continue;
        }

        let min_x = cmp::min(start_x, end_x);
        let max_x = cmp::max(start_x, end_x);
        let min_y = cmp::min(start_y, end_y);
        let max_y = cmp::max(start_y, end_y);

        // println!("{},{} -> {},{}", start_x, start_y, end_x, end_y);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                // println!("{},{}", x, y);
                grid[y as usize][x as usize] += 1;
            }
        }
    }

    count_overlaps(&grid)
}

fn get_direction(start: i32, end: i32) -> i32 {
    match start.cmp(&end) {
        cmp::Ordering::Greater => -1,
        cmp::Ordering::Equal => 0,
        cmp::Ordering::Less => 1,
    }
}

fn solve_b(input: &PuzzleInput) -> usize {
    let lines = parse_lines(input);
    let (max_x, max_y) = get_grid_size(&lines);
    let mut grid = vec![vec![0u32; max_x as usize + 1]; max_y as usize + 1];

    for line in &lines {
        let (start_x, start_y) = line.start;
        let (end_x, end_y) = line.end;

        // println!("{},{} -> {},{}", start_x, start_y, end_x, end_y);

        let mut x = start_x;
        let x_direction = get_direction(start_x, end_x);
        let mut y = start_y;
        let y_direction = get_direction(start_y, end_y);

        loop {
            // println!("{},{}", x, y);
            grid[y as usize][x as usize] += 1;
            if x == end_x && y == end_y {
                break;
            }
            x += x_direction;
            y += y_direction;
        }
    }

    count_overlaps(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9\n\
                              8,0 -> 0,8\n\
                              9,4 -> 3,4\n\
                              2,2 -> 2,1\n\
                              7,0 -> 7,4\n\
                              6,4 -> 2,0\n\
                              0,9 -> 2,9\n\
                              3,4 -> 1,4\n\
                              0,0 -> 8,8\n\
                              5,5 -> 8,2";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 5);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 12);
    }
}
