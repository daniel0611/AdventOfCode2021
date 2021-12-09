use std::collections::HashSet;
use aoc_utils::PuzzleInput;
const DAY: u8 = 9;

struct Basin {
    points: HashSet<(usize, usize)>,
}

impl Basin {
    fn new(low_point: (usize, usize), grid: &[Vec<usize>]) -> Basin {
        let mut basin = Basin {
            points: HashSet::new(),
        };
        basin.points.insert(low_point);
        basin.grow_area(grid, vec![&low_point]);

        basin
    }

    fn grow_new_points(&self, grid: &[Vec<usize>], search_points: Vec<&(usize, usize)>) -> HashSet<(usize, usize)> {
        let all_neighbors = search_points.iter().flat_map(|point| get_point_neighbors(grid, **point));
        let without_high_points = all_neighbors.filter(|(x,y)| grid[*y][*x] != 9);
        let new_points = without_high_points.filter(|point| !self.points.contains(point));
        new_points.collect()
    }

    fn grow_area(&mut self, grid: &[Vec<usize>], search_points: Vec<&(usize, usize)>) {
        let new_points = self.grow_new_points(grid, search_points);
        let new_points_count = new_points.len();
        self.points.extend(new_points.clone());

        // Stop if we haven't found anything new
        if new_points_count > 0 {
            self.grow_area(grid, new_points.iter().collect::<Vec<_>>());
        }
    }

    fn size(&self) -> usize {
        self.points.len()
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_grid(input: &PuzzleInput) -> Vec<Vec<usize>> {
    let lines = input.lines();

    let mut grid = vec![vec![0; lines[0].len()]; lines.len()];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c.to_digit(10).unwrap() as usize;
        }
    }

    grid
}

fn get_point_neighbors(grid: &[Vec<usize>], point: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = point;
    let mut neighbors = Vec::new();

    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < grid[0].len() - 1 {
        neighbors.push((x + 1, y));
    }

    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < grid.len() - 1 {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn find_low_points(grid: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut low_points = vec![];

    for y in 0..grid.len() {
        for x in 0..grid[y as usize].len() {
            let neighbors = get_point_neighbors(grid, (x, y));
            if neighbors.iter().all(|(ax, ay)| grid[*ay][*ax] > grid[y][x]) {
                low_points.push((x, y));
            }
        }
    }

    low_points
}

fn solve_a(input: &PuzzleInput) -> usize {
    let grid = parse_grid(input);
    let low_points = find_low_points(&grid);
    low_points
        .iter()
        .map(|(x, y)| 1 + grid[*y][*x])
        .sum()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let grid = parse_grid(input);
    let low_points = find_low_points(&grid);

    let basins = low_points.iter().map(|point| Basin::new(*point, &grid)).collect::<Vec<_>>();
    let mut basin_sizes = basins.iter().map(|b| b.size()).collect::<Vec<_>>();
    basin_sizes.sort_unstable(); // ascending
    basin_sizes.reverse(); // descending

    basin_sizes.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2199943210\n\
    3987894921\n\
    9856789892\n\
    8767896789\n\
    9899965678";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 15);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 1134);
    }
}
