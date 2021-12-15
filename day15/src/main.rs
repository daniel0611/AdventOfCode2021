use aoc_utils::PuzzleInput;
use std::collections::HashMap;
use std::collections::HashSet;
const DAY: u8 = 15;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
struct Point(usize, usize);

struct CaveMap {
    map: Vec<Vec<u32>>,
}

impl CaveMap {
    fn parse(input: &PuzzleInput) -> CaveMap {
        CaveMap {
            map: input
                .lines()
                .iter()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }

    fn get_safest_path(&self) -> Vec<Point> {
        // I have no idea what I'm doing.
        // https://en.wikipedia.org/wiki/A*_search_algorithm

        let start = Point(0, 0);
        let goal = Point(self.map.len() - 1, self.map[0].len() - 1);

        let mut open_set = HashSet::new();
        open_set.insert(start);

        let mut came_from: HashMap<Point, Point> = HashMap::new();

        let mut g_score = HashMap::new();
        g_score.insert(start, 0);

        let mut f_score = HashMap::new();
        f_score.insert(start, Self::manhattan_distance(&start, &goal));

        loop {
            if open_set.is_empty() {
                panic!("No path found");
            }

            let current = *open_set
                .iter()
                .min_by_key(|p| f_score.get(p).or(Some(&u32::max_value())))
                .unwrap();
            if current == goal {
                return Self::reconstruct_path(&came_from, current);
            }

            open_set.remove(&current);

            for neighbor in self.get_neighbors(&current) {
                let g_score_current = *g_score.entry(current).or_insert(u32::max_value());
                let g_score_neighbor = *g_score.entry(neighbor).or_insert(u32::max_value());

                let tentative_g_score = g_score_current + self.map[neighbor.1][neighbor.0];
                if tentative_g_score < g_score_neighbor {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(
                        neighbor,
                        tentative_g_score + Self::manhattan_distance(&neighbor, &goal),
                    );

                    open_set.insert(neighbor);
                }
            }
        }
    }

    fn reconstruct_path(came_from: &HashMap<Point, Point>, current: Point) -> Vec<Point> {
        let mut curr = current;
        let mut path = vec![curr];
        loop {
            if came_from.contains_key(&curr) {
                curr = came_from[&curr];
                path.push(curr);
            } else {
                path.reverse();
                return path;
            }
        }
    }

    fn manhattan_distance(a: &Point, b: &Point) -> u32 {
        (a.0 as isize - b.0 as isize).abs() as u32 + (a.1 as isize - b.1 as isize).abs() as u32
    }

    fn get_neighbors(&self, p: &Point) -> Vec<Point> {
        let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        let x = p.0 as isize;
        let y = p.1 as isize;

        offsets
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|(x, _)| *x >= 0 && *x < self.map[0].len() as isize)
            .filter(|(_, y)| *y >= 0 && *y < self.map.len() as isize)
            .map(|(x, y)| Point(x as usize, y as usize))
            .collect()
    }

    fn get_path_risk(&self, path: &[Point]) -> usize {
        path.iter()
            .skip(1)
            .map(|p| self.map[p.1][p.0] as usize)
            .sum()
    }

    fn expand_part_b(&mut self) {
        let mut extended_map = vec![];

        for y in 0..self.map.len() * 5 {
            let mut row = vec![];

            for x in 0..self.map[y % self.map.len()].len() * 5 {
                let v = self.map[y % self.map.len()][x % self.map[0].len()];
                let mut v = v + (x / self.map[0].len()) as u32 + (y / self.map.len()) as u32;
                if v > 9 {
                    v -= 9
                }
                row.push(v);
            }

            extended_map.push(row);
        }

        self.map = extended_map;
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    let map = CaveMap::parse(input);
    let path = map.get_safest_path();

    // println!();
    // for y in 0..map.map.len() {
    //     for x in 0..map.map[y].len() {
    //         if path.contains(&Point(x, y)) {
    //             print!("\x1b[1m{}\x1b[22m", map.map[y][x]);
    //         } else {
    //             print!("{}", map.map[y][x]);
    //         }
    //     }
    //     println!();
    // }

    map.get_path_risk(&path)
}

fn solve_b(input: &PuzzleInput) -> usize {
    let mut map = CaveMap::parse(input);
    map.expand_part_b();
    let path = map.get_safest_path();
    map.get_path_risk(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1163751742\n\
                              1381373672\n\
                              2136511328\n\
                              3694931569\n\
                              7463417111\n\
                              1319128137\n\
                              1359912421\n\
                              3125421639\n\
                              1293138521\n\
                              2311944581";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        // Too slow, but still gets run in CI in release mode
        // solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 40);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 315);
    }
}
