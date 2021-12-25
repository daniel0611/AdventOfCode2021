use aoc_utils::PuzzleInput;
const DAY: u8 = 25;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("Day 25: {}", solve(&input));
}

#[derive(PartialEq, Eq, Clone)]
enum SeaCucumber {
    None,
    East,
    South,
}

fn parse_map(input: &PuzzleInput) -> Vec<Vec<SeaCucumber>> {
    input
        .lines()
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => SeaCucumber::None,
                    '>' => SeaCucumber::East,
                    'v' => SeaCucumber::South,
                    _ => panic!("Invalid character {}", c),
                })
                .collect()
        })
        .collect()
}

fn simulate_step(map: &[Vec<SeaCucumber>]) -> Vec<Vec<SeaCucumber>> {
    let old_map = map.to_vec();
    let mut new_map = old_map.clone();

    // Move east
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if old_map[y][x] == SeaCucumber::East {
                let new_idx = (x + 1) % old_map[y].len();
                if old_map[y][new_idx] == SeaCucumber::None {
                    new_map[y][new_idx] = SeaCucumber::East;
                    new_map[y][x] = SeaCucumber::None;
                }
            }
        }
    }

    let old_map = new_map;
    let mut new_map = old_map.clone();

    // Move south
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if old_map[y][x] == SeaCucumber::South {
                let new_idx = (y + 1) % old_map.len();
                if old_map[new_idx][x] == SeaCucumber::None {
                    new_map[new_idx][x] = SeaCucumber::South;
                    new_map[y][x] = SeaCucumber::None;
                }
            }
        }
    }

    new_map
}

fn solve(input: &PuzzleInput) -> usize {
    let mut map = parse_map(input);
    let mut step = 0;

    loop {
        let new_map = simulate_step(&map);
        step += 1;
        println!("Step {}", step);

        let has_changed = new_map.iter().zip(map.iter()).any(|(new_row, old_row)| {
            new_row
                .iter()
                .zip(old_row.iter())
                .any(|(new_cell, old_cell)| new_cell != old_cell)
        });
        if !has_changed {
            return step;
        }

        map = new_map;
        if step >= 20000 {
            panic!("Too many steps");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "v...>>.vv>\n\
                              .vv>>.vv..\n\
                              >>.>v>...v\n\
                              >>v>>.>.v.\n\
                              v>v.vv.v..\n\
                              >.>>..v...\n\
                              .vv..>.>v.\n\
                              v.v..>>v.v\n\
                              ....v..v.>";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve(&input);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(&PuzzleInput::new(TEST_INPUT)), 58);
    }
}
