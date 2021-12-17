use aoc_utils::PuzzleInput;
use regex::Regex;
use std::cmp::Ordering;

const DAY: u8 = 17;

struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl TargetArea {
    fn parse(input: &PuzzleInput) -> TargetArea {
        let re =
            Regex::new("target area: x=(-*[0-9]+)\\.\\.(-*[0-9]+), y=(-*[0-9]+)\\.\\.(-*[0-9]+)")
                .unwrap();
        let caps = re.captures(input.raw_input.trim()).unwrap();

        TargetArea {
            x_min: caps[1].parse::<i32>().unwrap(),
            x_max: caps[2].parse::<i32>().unwrap(),
            y_min: caps[3].parse::<i32>().unwrap(),
            y_max: caps[4].parse::<i32>().unwrap(),
        }
    }
}

fn calculate_trajectory(initial_velocity: (i32, i32), target_area: &TargetArea) -> Vec<(i32, i32)> {
    let mut steps = vec![(0, 0)];
    let mut velocity = initial_velocity;

    loop {
        let (x, y) = steps.last().unwrap();
        let (x, y) = (x + velocity.0, y + velocity.1);
        if x > target_area.x_max || y < target_area.y_min {
            break;
        }

        steps.push((x, y));

        // Drag
        match velocity.0.cmp(&0) {
            Ordering::Greater => velocity.0 -= 1,
            Ordering::Less => velocity.0 += 1,
            Ordering::Equal => (),
        }
        // Gravity
        velocity.1 -= 1;
    }

    steps
}

fn get_trajectory_height(trajectory: &[(i32, i32)]) -> i32 {
    trajectory.iter().map(|p| p.1).max().unwrap()
}

fn trajectory_hits_target(trajectory: &[(i32, i32)], target_area: &TargetArea) -> bool {
    trajectory.iter().any(|p| {
        p.0 >= target_area.x_min
            && p.0 <= target_area.x_max
            && p.1 >= target_area.y_min
            && p.1 <= target_area.y_max
    })
    //let end_point = trajectory.last().unwrap();
    //end_point.0 >= target_area.x_min && end_point.1 <= target_area.y_max
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> i32 {
    let target_area = TargetArea::parse(input);
    let mut heights = vec![];

    for x_velocity in 0..=target_area.x_max {
        for y_velocity in -100..250 {
            let trajectory = calculate_trajectory((x_velocity, y_velocity), &target_area);
            if trajectory_hits_target(&trajectory, &target_area) {
                heights.push(get_trajectory_height(&trajectory));
            }
        }
    }

    *heights.iter().max().unwrap()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let target_area = TargetArea::parse(input);
    let mut initial_velocities = vec![];

    for x_velocity in 0..=target_area.x_max {
        for y_velocity in -200..250 {
            let trajectory = calculate_trajectory((x_velocity, y_velocity), &target_area);
            if trajectory_hits_target(&trajectory, &target_area) {
                initial_velocities.push((x_velocity, y_velocity));
            }
        }
    }

    initial_velocities.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_a_basic_trajectory() {
        let target_area = TargetArea::parse(&PuzzleInput::new(TEST_INPUT));
        let trajectory = calculate_trajectory((7, 2), &target_area);
        assert_eq!(
            trajectory,
            vec![
                (0, 0),
                (7, 2),
                (13, 3),
                (18, 3),
                (22, 2),
                (25, 0),
                (27, -3),
                (28, -7)
            ]
        );
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 45);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 112);
    }
}
