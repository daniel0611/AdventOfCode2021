use aoc_utils::PuzzleInput;
const DAY: u8 = 13;

struct TransparentPaper {
    map: Vec<Vec<bool>>,
    fold_instructions: Vec<FoldInstruction>,
}

impl TransparentPaper {
    fn parse(input: &PuzzleInput) -> TransparentPaper {
        let mut parts = input.raw_input.split("\n\n");
        let coordinate_lines = parts.next().unwrap().split('\n');
        let instruction_lines = parts.next().unwrap().split('\n');

        let coordinates = coordinate_lines
            .map(|l| {
                let mut parts = l.split(',');
                let x = parts.next().unwrap().parse::<usize>().unwrap();
                let y = parts.next().unwrap().parse::<usize>().unwrap();
                (x, y)
            })
            .collect::<Vec<_>>();

        let x_max = coordinates.iter().map(|(x, _)| *x).max().unwrap();
        let y_max = coordinates.iter().map(|(_, y)| *y).max().unwrap();
        let mut map = vec![vec![false; x_max + 1]; y_max + 1];

        for (x, y) in coordinates {
            map[y][x] = true;
        }

        let instructions = instruction_lines
            .map(|l| FoldInstruction::parse(l))
            .collect::<Vec<_>>();

        TransparentPaper {
            map,
            fold_instructions: instructions,
        }
    }

    fn execute_fold(&mut self, fold: &FoldInstruction) {
        if fold.direction_y {
            let mut new_map = vec![vec![false; self.map[0].len()]; fold.coordinate];

            for (upper_y_index, row) in new_map.iter_mut().enumerate().take(fold.coordinate) {
                let lower_y_index = 2 * fold.coordinate - upper_y_index;
                if upper_y_index >= lower_y_index {
                    break;
                }

                for (x, point) in row.iter_mut().enumerate().take(self.map[0].len()) {
                    if lower_y_index < self.map.len() {
                        *point = self.map[upper_y_index][x] || self.map[lower_y_index][x];
                    } else {
                        *point = self.map[upper_y_index][x];
                    }
                }
            }

            self.map = new_map;
        } else {
            let mut new_map = vec![vec![false; fold.coordinate]; self.map.len()];

            for left_x_index in 0..fold.coordinate {
                let right_x_index = 2 * fold.coordinate - left_x_index;

                for (y, row) in new_map.iter_mut().enumerate().take(self.map.len()) {
                    if right_x_index < self.map[0].len() {
                        row[left_x_index] = self.map[y][left_x_index] || self.map[y][right_x_index];
                    } else {
                        row[left_x_index] = self.map[y][left_x_index];
                    }
                }
            }

            self.map = new_map;
        }
    }

    fn count_dots(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }
}

#[derive(Clone)]
struct FoldInstruction {
    direction_y: bool,
    coordinate: usize,
}

impl FoldInstruction {
    fn parse(line: &str) -> FoldInstruction {
        let parts = line.split(' ');
        let mut coordinate_parts = parts.last().unwrap().split('=');

        FoldInstruction {
            direction_y: coordinate_parts.next().unwrap() == "y",
            coordinate: coordinate_parts.next().unwrap().parse().unwrap(),
        }
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> usize {
    let mut paper = TransparentPaper::parse(input);
    let instruction = paper.fold_instructions[0].clone();
    println!("{} {}", paper.map.len(), paper.map[0].len());
    paper.execute_fold(&instruction);
    println!("{} {}", paper.map.len(), paper.map[0].len());
    paper.count_dots()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let mut paper = TransparentPaper::parse(input);

    for instruction in paper.fold_instructions.clone().iter() {
        paper.execute_fold(instruction);
    }

    for y in 0..paper.map.len() {
        for x in 0..paper.map[0].len() {
            print!("{}", if paper.map[y][x] { '#' } else { '.' });
        }
        println!();
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "6,10\n\
    0,14\n\
    9,10\n\
    0,3\n\
    10,4\n\
    4,11\n\
    6,0\n\
    6,12\n\
    4,1\n\
    0,13\n\
    10,12\n\
    3,4\n\
    3,0\n\
    8,4\n\
    1,10\n\
    2,14\n\
    8,10\n\
    9,0\n\
    \n\
    fold along y=7\n\
    fold along x=5";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 17);
    }
}
