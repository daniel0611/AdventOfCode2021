use aoc_utils::PuzzleInput;
use std::collections::HashSet;
const DAY: u8 = 12;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_map(input: &PuzzleInput) -> Vec<(String, String)> {
    let mut map = vec![];
    for line in input.lines() {
        let mut parts = line.split('-');
        let key = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        map.push((key, value));
    }
    map
}

fn get_routes(map: &[(String, String)], path: Vec<String>, part_b: bool) -> Vec<Vec<String>> {
    let current_point = path.last().unwrap();
    if current_point == "end" {
        return vec![path];
    }

    let mut paths = vec![];

    for (point1, point2) in map.iter() {
        if point1 == current_point || point2 == current_point {
            let other_point = if point1 == current_point {
                point2.clone()
            } else {
                point1.clone()
            };

            // We don't want to go back to the beginning lol
            if other_point == "start" {
                continue;
            }

            if other_point == other_point.to_lowercase() {
                if !part_b {
                    // A: just check if we already were there
                    if path.contains(&other_point) {
                        continue;
                    }
                } else {
                    // B: we may go to one small cave twice
                    if visited_small_cave_twice(&path, &other_point) && path.contains(&other_point)
                    {
                        continue;
                    }
                }
            }

            let mut p = path.clone();
            p.push(other_point);
            let new_paths = get_routes(map, p, part_b);
            paths.extend(new_paths);
        }
    }

    paths
}

fn visited_small_cave_twice(path: &[String], point: &str) -> bool {
    let mut path_with_new_cave = path.to_owned();
    path_with_new_cave.push(point.to_string());

    let small_caves = path
        .iter()
        .filter(|p| **p == p.to_lowercase())
        .collect::<Vec<_>>();
    let unique_caves = small_caves.iter().collect::<HashSet<_>>();

    for cave in unique_caves.iter() {
        if small_caves.iter().filter(|c| c == cave).count() > 1 {
            return true;
        }
    }

    false
}

fn get_routes_count(input: &PuzzleInput, part_b: bool) -> usize {
    let map = parse_map(input);
    let routes = get_routes(&map, vec!["start".to_string()], part_b);
    routes.len()
}

fn solve_a(input: &PuzzleInput) -> usize {
    get_routes_count(input, false)
}

fn solve_b(input: &PuzzleInput) -> usize {
    get_routes_count(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "start-A\n\
    start-b\n\
    A-c\n\
    A-b\n\
    b-d\n\
    A-end\n\
    b-end";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        // B takes a bit to long to run every time
        // CI still runs it though in the separate run step
        // solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 10);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 36);
    }
}
