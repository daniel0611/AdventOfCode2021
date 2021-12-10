use aoc_utils::PuzzleInput;
const DAY: u8 = 4;

#[derive(Clone, PartialEq, Eq)]
struct BingoBoard {
    pub rows: Vec<Vec<u32>>,
}

impl BingoBoard {
    fn new(input: &[&str]) -> Self {
        BingoBoard {
            rows: input
                .iter()
                .map(|row| {
                    row.split(' ')
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect::<Vec<Vec<_>>>(),
        }
    }

    fn check_for_win(&self, numbers: &[u32]) -> bool {
        self.check_rows(numbers) || self.check_columns(numbers)
    }

    fn check_rows(&self, numbers: &[u32]) -> bool {
        self.rows
            .iter()
            .any(|row| row.iter().all(|n| numbers.contains(n)))
    }

    fn check_columns(&self, numbers: &[u32]) -> bool {
        let mut columns = vec![vec![]; self.rows.len()];
        for (_i, row) in self.rows.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                columns[j].push(*n);
            }
        }
        columns
            .iter()
            .any(|column| column.iter().all(|n| numbers.contains(n)))
    }

    fn calculate_score(&self, numbers: &[u32]) -> u32 {
        let mut unchecked = self
            .rows
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect::<Vec<_>>();

        for n in numbers {
            unchecked.retain(|&x| x != *n);
        }

        let unchecked_sum: u32 = unchecked.iter().sum();
        let last_called = numbers.iter().last().unwrap();
        unchecked_sum * last_called
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_input(input: &PuzzleInput) -> (Vec<u32>, Vec<BingoBoard>) {
    let parts = input.raw_input.split("\n\n").collect::<Vec<_>>();

    let bingo_numbers: Vec<u32> = parts
        .get(0)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let bingo_boards: Vec<BingoBoard> = parts
        .iter()
        .skip(1)
        .map(|b| BingoBoard::new(&b.to_string().split('\n').collect::<Vec<_>>()))
        .collect();

    (bingo_numbers, bingo_boards)
}

fn solve_a(input: &PuzzleInput) -> u32 {
    let (bingo_numbers, bingo_boards) = parse_input(input);

    for i in 1..=bingo_numbers.len() {
        let numbers = bingo_numbers.iter().cloned().take(i).collect::<Vec<_>>();
        for board in &bingo_boards {
            if board.check_for_win(&numbers) {
                return board.calculate_score(&numbers);
            }
        }
    }

    panic!("No board ever wins?");
}

fn solve_b(input: &PuzzleInput) -> u32 {
    let (bingo_numbers, mut bingo_boards) = parse_input(input);

    for i in 1..=bingo_numbers.len() {
        let numbers = bingo_numbers.iter().cloned().take(i).collect::<Vec<_>>();
        let mut new_boards = bingo_boards.clone();

        for board in &bingo_boards {
            if board.check_for_win(&numbers) {
                new_boards.retain(|b| b != board);

                // This was the last board to win
                if new_boards.is_empty() {
                    return board.calculate_score(&numbers);
                }
            }
        }

        bingo_boards = new_boards;
    }

    panic!("Multiple boards won at the same time last?");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
                               \n\
                               22 13 17 11  0\n\
                               8  2 23  4 24\n\
                               21  9 14 16  7\n\
                               6 10  3 18  5\n\
                               1 12 20 15 19\n\
                               \n\
                               3 15  0  2 22\n\
                               9 18 13 17  5\n\
                               19  8  7 25 23\n\
                               20 11 10 24  4\n\
                               14 21 16 12  6\n\
                               \n\
                               14 21 17 24  4\n\
                               10 16 15  9 19\n\
                               18  8 23 26 20\n\
                               22 11 13  6  5\n\
                               2  0 12  3  7";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 4512);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 1924);
    }
}
